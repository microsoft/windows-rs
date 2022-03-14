#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AbortDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
        }
        ::core::mem::transmute(AbortDoc(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_CAPABILITIES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(12u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(6u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLLATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(22u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(32u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COPIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(18u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DRIVER: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(11u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DUPLEX: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(7u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(13u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_EXTRA: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(9u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FIELDS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(1u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(14u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(5u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(29u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(34u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(35u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(4u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(17u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_NUP: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(33u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(16u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(2u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(3u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(25u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(28u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(26u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(31u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(27u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_SIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(8u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_STAPLE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(30u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(15u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_VERSION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(10u32);
impl ::core::marker::Copy for DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVICE_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: ::windows::core::PCSTR,
    pub lpszOutput: ::windows::core::PCSTR,
    pub lpszDatatype: ::windows::core::PCSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOA {}
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOCINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOA").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
unsafe impl ::windows::core::Abi for DOCINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOCINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOCINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOCINFOA {}
impl ::core::default::Default for DOCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: ::windows::core::PCWSTR,
    pub lpszOutput: ::windows::core::PCWSTR,
    pub lpszDatatype: ::windows::core::PCWSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOW {}
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOCINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOW").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
unsafe impl ::windows::core::Abi for DOCINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOCINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOCINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOCINFOW {}
impl ::core::default::Default for DOCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRAWPATRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRAWPATRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWPATRECT").field("ptPosition", &self.ptPosition).field("ptSize", &self.ptSize).field("wStyle", &self.wStyle).field("wPattern", &self.wPattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRAWPATRECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRAWPATRECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRAWPATRECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceCapabilitiesA(pdevice: ::windows::core::PCSTR, pport: ::windows::core::PCSTR, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32;
        }
        ::core::mem::transmute(DeviceCapabilitiesA(pdevice.into_param().abi(), pport.into_param().abi(), ::core::mem::transmute(fwcapability), ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceCapabilitiesW(pdevice: ::windows::core::PCWSTR, pport: ::windows::core::PCWSTR, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32;
        }
        ::core::mem::transmute(DeviceCapabilitiesW(pdevice.into_param().abi(), pport.into_param().abi(), ::core::mem::transmute(fwcapability), ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
        }
        ::core::mem::transmute(EndDoc(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
        }
        ::core::mem::transmute(EndPage(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn Escape<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdc: Param0, iescape: i32, cjin: i32, pvin: Param3, pvout: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Escape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjin: i32, pvin: ::windows::core::PCSTR, pvout: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(Escape(hdc.into_param().abi(), ::core::mem::transmute(iescape), ::core::mem::transmute(cjin), pvin.into_param().abi(), ::core::mem::transmute(pvout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtEscape<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdc: Param0, iescape: i32, cjinput: i32, lpindata: Param3, cjoutput: i32, lpoutdata: ::windows::core::PSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtEscape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjinput: i32, lpindata: ::windows::core::PCSTR, cjoutput: i32, lpoutdata: ::windows::core::PSTR) -> i32;
        }
        ::core::mem::transmute(ExtEscape(hdc.into_param().abi(), ::core::mem::transmute(iescape), ::core::mem::transmute(cjinput), lpindata.into_param().abi(), ::core::mem::transmute(cjoutput), ::core::mem::transmute(lpoutdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HPTPROVIDER(pub isize);
impl HPTPROVIDER {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPTPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPTPROVIDER {}
impl ::core::fmt::Debug for HPTPROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPTPROVIDER").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HPTPROVIDER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, discardcontrolpartname: Param1) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXpsOMPackageWriter)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXpsOMFactory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMObjectFactory>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetXpsType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXpsType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
}
impl ::core::convert::From<IXpsDocumentPackageTarget> for ::windows::core::IUnknown {
    fn from(value: IXpsDocumentPackageTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsDocumentPackageTarget> for ::windows::core::IUnknown {
    fn from(value: &IXpsDocumentPackageTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsDocumentPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsDocumentPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsDocumentPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentPackageTarget {}
impl ::core::fmt::Debug for IXpsDocumentPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentPackageTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget {
    type Vtable = IXpsDocumentPackageTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0b6d38_53ad_41da_b212_d37637a6714e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget3D(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter3D<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, documentsequencepartname: Param0, discardcontrolpartname: Param1, modelpartname: Param2, modeldata: Param3) -> ::windows::core::Result<IXpsOMPackageWriter3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXpsOMPackageWriter3D)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), modelpartname.into_param().abi(), modeldata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXpsOMFactory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMObjectFactory>(result__)
    }
}
impl ::core::convert::From<IXpsDocumentPackageTarget3D> for ::windows::core::IUnknown {
    fn from(value: IXpsDocumentPackageTarget3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsDocumentPackageTarget3D> for ::windows::core::IUnknown {
    fn from(value: &IXpsDocumentPackageTarget3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsDocumentPackageTarget3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsDocumentPackageTarget3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsDocumentPackageTarget3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentPackageTarget3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentPackageTarget3D {}
impl ::core::fmt::Debug for IXpsDocumentPackageTarget3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentPackageTarget3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget3D {
    type Vtable = IXpsDocumentPackageTarget3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60ba71b8_3101_4984_9199_f4ea775ff01d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3D_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMBrush(::windows::core::IUnknown);
impl IXpsOMBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
}
impl ::core::convert::From<IXpsOMBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMBrush> for IXpsOMShareable {
    fn from(value: IXpsOMBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMBrush {}
impl ::core::fmt::Debug for IXpsOMBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMBrush {
    type Vtable = IXpsOMBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrush_Vtbl {
    pub base: IXpsOMShareable_Vtbl,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMCanvas(::windows::core::IUnknown);
impl IXpsOMCanvas {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLocal)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisuals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUseAliasedEdgeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseAliasedEdgeMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, usealiasededgemode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseAliasedEdgeMode)(::core::mem::transmute_copy(self), usealiasededgemode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessibilityShortDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, shortdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccessibilityShortDescription)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessibilityLongDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, longdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccessibilityLongDescription)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionaryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictionaryLocal)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionaryResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictionaryResource)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
    }
}
impl ::core::convert::From<IXpsOMCanvas> for ::windows::core::IUnknown {
    fn from(value: IXpsOMCanvas) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMCanvas> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMCanvas) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMCanvas> for IXpsOMShareable {
    fn from(value: IXpsOMCanvas) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMCanvas> for IXpsOMShareable {
    fn from(value: &IXpsOMCanvas) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMCanvas> for IXpsOMVisual {
    fn from(value: IXpsOMCanvas) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMCanvas> for IXpsOMVisual {
    fn from(value: &IXpsOMCanvas) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &'a IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMCanvas {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMCanvas {}
impl ::core::fmt::Debug for IXpsOMCanvas {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMCanvas").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMCanvas {
    type Vtable = IXpsOMCanvas_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvas_Vtbl {
    pub base: IXpsOMVisual_Vtbl,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseAliasedEdgeMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseAliasedEdgeMode: usize,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMColorProfileResource(::windows::core::IUnknown);
impl IXpsOMColorProfileResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMColorProfileResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMColorProfileResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMColorProfileResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMColorProfileResource> for IXpsOMPart {
    fn from(value: IXpsOMColorProfileResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResource> for IXpsOMPart {
    fn from(value: &IXpsOMColorProfileResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMColorProfileResource> for IXpsOMResource {
    fn from(value: IXpsOMColorProfileResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResource> for IXpsOMResource {
    fn from(value: &IXpsOMColorProfileResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMColorProfileResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMColorProfileResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMColorProfileResource {}
impl ::core::fmt::Debug for IXpsOMColorProfileResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMColorProfileResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResource {
    type Vtable = IXpsOMColorProfileResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMColorProfileResourceCollection(::windows::core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByPartName)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMColorProfileResourceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMColorProfileResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMColorProfileResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMColorProfileResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMColorProfileResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMColorProfileResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMColorProfileResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMColorProfileResourceCollection {}
impl ::core::fmt::Debug for IXpsOMColorProfileResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMColorProfileResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResourceCollection {
    type Vtable = IXpsOMColorProfileResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12759630_5fba_4283_8f7d_cca849809edb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMCoreProperties(::windows::core::IUnknown);
impl IXpsOMCoreProperties {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetCategory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, category: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCategory)(::core::mem::transmute_copy(self), category.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetContentStatus(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContentStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetContentStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, contentstatus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContentStatus)(::core::mem::transmute_copy(self), contentstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, contenttype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContentType)(::core::mem::transmute_copy(self), contenttype.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreated(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(created)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCreator(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCreator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetCreator<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, creator: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreator)(::core::mem::transmute_copy(self), creator.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIdentifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetIdentifier<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, identifier: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIdentifier)(::core::mem::transmute_copy(self), identifier.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetKeywords(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetKeywords)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetKeywords<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, keywords: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKeywords)(::core::mem::transmute_copy(self), keywords.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLastModifiedBy(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastModifiedBy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLastModifiedBy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lastmodifiedby: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastModifiedBy)(::core::mem::transmute_copy(self), lastmodifiedby.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastPrinted(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPrinted)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastPrinted)(::core::mem::transmute_copy(self), ::core::mem::transmute(lastprinted)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(modified)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRevision)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetRevision<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, revision: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRevision)(::core::mem::transmute_copy(self), revision.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSubject(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, subject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubject)(::core::mem::transmute_copy(self), subject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTitle(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, title: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTitle)(::core::mem::transmute_copy(self), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVersion)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
}
impl ::core::convert::From<IXpsOMCoreProperties> for ::windows::core::IUnknown {
    fn from(value: IXpsOMCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMCoreProperties> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMCoreProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMCoreProperties> for IXpsOMPart {
    fn from(value: IXpsOMCoreProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMCoreProperties> for IXpsOMPart {
    fn from(value: &IXpsOMCoreProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMCoreProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMCoreProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMCoreProperties {}
impl ::core::fmt::Debug for IXpsOMCoreProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMCoreProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMCoreProperties {
    type Vtable = IXpsOMCoreProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCoreProperties_Vtbl {
    pub base: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreated: usize,
    pub GetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetModified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModified: usize,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDashCollection(::windows::core::IUnknown);
impl IXpsOMDashCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<XPS_DASH> {
        let mut result__: XPS_DASH = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DASH>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), ::core::mem::transmute(dash)).ok()
    }
}
impl ::core::convert::From<IXpsOMDashCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDashCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDashCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDashCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDashCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDashCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDashCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDashCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDashCollection {}
impl ::core::fmt::Debug for IXpsOMDashCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDashCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDashCollection {
    type Vtable = IXpsOMDashCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDictionary(::windows::core::IUnknown);
impl IXpsOMDictionary {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32, key: *mut ::windows::core::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(key), ::core::mem::transmute(entry)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetByKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, beforeentry: Param1) -> ::windows::core::Result<IXpsOMShareable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByKey)(::core::mem::transmute_copy(self), key.into_param().abi(), beforeentry.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMShareable>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetIndex<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, entry: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIndex)(::core::mem::transmute_copy(self), entry.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, entry: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
}
impl ::core::convert::From<IXpsOMDictionary> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDictionary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDictionary> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDictionary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDictionary {}
impl ::core::fmt::Debug for IXpsOMDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDictionary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDictionary {
    type Vtable = IXpsOMDictionary_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDictionary_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: *mut ::windows::core::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetByKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows::core::PCWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows::core::PCWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocument(::windows::core::IUnknown);
impl IXpsOMDocument {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPageReferences(&self) -> ::windows::core::Result<IXpsOMPageReferenceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPageReferences)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReferenceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPrintTicketResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrintTicketResource)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentStructureResource(&self) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentStructureResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>>(&self, documentstructureresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentStructureResource)(::core::mem::transmute_copy(self), documentstructureresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureBlockResources(&self) -> ::windows::core::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureBlockResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResourceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
}
impl ::core::convert::From<IXpsOMDocument> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocument> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMDocument> for IXpsOMPart {
    fn from(value: IXpsOMDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocument> for IXpsOMPart {
    fn from(value: &IXpsOMDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocument {}
impl ::core::fmt::Debug for IXpsOMDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDocument {
    type Vtable = IXpsOMDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocument_Vtbl {
    pub base: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPageReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSignatureBlockResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentCollection(::windows::core::IUnknown);
impl IXpsOMDocumentCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, document: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), document.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMDocumentCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDocumentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDocumentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDocumentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDocumentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDocumentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentCollection {}
impl ::core::fmt::Debug for IXpsOMDocumentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentCollection {
    type Vtable = IXpsOMDocumentCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1c87f0d_e947_4754_8a25_971478f7e83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentSequence(::windows::core::IUnknown);
impl IXpsOMDocumentSequence {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<IXpsOMDocumentCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocuments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPrintTicketResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrintTicketResource)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMDocumentSequence> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDocumentSequence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentSequence> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDocumentSequence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMDocumentSequence> for IXpsOMPart {
    fn from(value: IXpsOMDocumentSequence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentSequence> for IXpsOMPart {
    fn from(value: &IXpsOMDocumentSequence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDocumentSequence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentSequence {}
impl ::core::fmt::Debug for IXpsOMDocumentSequence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentSequence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentSequence {
    type Vtable = IXpsOMDocumentSequence_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequence_Vtbl {
    pub base: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentStructureResource(::windows::core::IUnknown);
impl IXpsOMDocumentStructureResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMDocumentStructureResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMDocumentStructureResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentStructureResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMDocumentStructureResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMDocumentStructureResource> for IXpsOMPart {
    fn from(value: IXpsOMDocumentStructureResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentStructureResource> for IXpsOMPart {
    fn from(value: &IXpsOMDocumentStructureResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMDocumentStructureResource> for IXpsOMResource {
    fn from(value: IXpsOMDocumentStructureResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMDocumentStructureResource> for IXpsOMResource {
    fn from(value: &IXpsOMDocumentStructureResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMDocumentStructureResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentStructureResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentStructureResource {}
impl ::core::fmt::Debug for IXpsOMDocumentStructureResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentStructureResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentStructureResource {
    type Vtable = IXpsOMDocumentStructureResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85febc8a_6b63_48a9_af07_7064e4ecff30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentStructureResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMFontResource(::windows::core::IUnknown);
impl IXpsOMFontResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, embeddingoption: XPS_FONT_EMBEDDING, partname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(embeddingoption), partname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetEmbeddingOption(&self) -> ::windows::core::Result<XPS_FONT_EMBEDDING> {
        let mut result__: XPS_FONT_EMBEDDING = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEmbeddingOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_FONT_EMBEDDING>(result__)
    }
}
impl ::core::convert::From<IXpsOMFontResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMFontResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMFontResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMFontResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMFontResource> for IXpsOMPart {
    fn from(value: IXpsOMFontResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMFontResource> for IXpsOMPart {
    fn from(value: &IXpsOMFontResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMFontResource> for IXpsOMResource {
    fn from(value: IXpsOMFontResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMFontResource> for IXpsOMResource {
    fn from(value: &IXpsOMFontResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMFontResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMFontResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMFontResource {}
impl ::core::fmt::Debug for IXpsOMFontResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMFontResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMFontResource {
    type Vtable = IXpsOMFontResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMFontResourceCollection(::windows::core::IUnknown);
impl IXpsOMFontResourceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByPartName)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMFontResourceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMFontResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMFontResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMFontResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMFontResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMFontResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMFontResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMFontResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMFontResourceCollection {}
impl ::core::fmt::Debug for IXpsOMFontResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMFontResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMFontResourceCollection {
    type Vtable = IXpsOMFontResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometry(::windows::core::IUnknown);
impl IXpsOMGeometry {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFigures(&self) -> ::windows::core::Result<IXpsOMGeometryFigureCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFigures)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigureCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillRule(&self) -> ::windows::core::Result<XPS_FILL_RULE> {
        let mut result__: XPS_FILL_RULE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillRule)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_FILL_RULE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillRule)(::core::mem::transmute_copy(self), ::core::mem::transmute(fillrule)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLookup)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
}
impl ::core::convert::From<IXpsOMGeometry> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGeometry> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMGeometry> for IXpsOMShareable {
    fn from(value: IXpsOMGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGeometry> for IXpsOMShareable {
    fn from(value: &IXpsOMGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometry {}
impl ::core::fmt::Debug for IXpsOMGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGeometry {
    type Vtable = IXpsOMGeometry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometry_Vtbl {
    pub base: IXpsOMShareable_Vtbl,
    pub GetFigures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometryFigure(::windows::core::IUnknown);
impl IXpsOMGeometryFigure {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSegmentData)(::core::mem::transmute_copy(self), ::core::mem::transmute(datacount), ::core::mem::transmute(segmentdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSegmentTypes)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmenttypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSegmentStrokes)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentstrokes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSegments)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentdatacount), ::core::mem::transmute(segmenttypes), ::core::mem::transmute(segmentdata), ::core::mem::transmute(segmentstrokes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStartPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsClosed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isclosed: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsClosed)(::core::mem::transmute_copy(self), isclosed.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsFilled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsFilled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsFilled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isfilled: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsFilled)(::core::mem::transmute_copy(self), isfilled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSegmentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSegmentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSegmentDataCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSegmentDataCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSegmentStrokePattern(&self) -> ::windows::core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__: XPS_SEGMENT_STROKE_PATTERN = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSegmentStrokePattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SEGMENT_STROKE_PATTERN>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
}
impl ::core::convert::From<IXpsOMGeometryFigure> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGeometryFigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGeometryFigure> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGeometryFigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGeometryFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGeometryFigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGeometryFigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometryFigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometryFigure {}
impl ::core::fmt::Debug for IXpsOMGeometryFigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometryFigure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigure {
    type Vtable = IXpsOMGeometryFigure_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd410dc83_908c_443e_8947_b1795d3c165a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigure_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSegmentStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSegmentStrokes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSegments: usize,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsFilled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsFilled: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometryFigureCollection(::windows::core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, geometryfigure: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), geometryfigure.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMGeometryFigureCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGeometryFigureCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGeometryFigureCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGeometryFigureCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGeometryFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGeometryFigureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGeometryFigureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometryFigureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometryFigureCollection {}
impl ::core::fmt::Debug for IXpsOMGeometryFigureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometryFigureCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigureCollection {
    type Vtable = IXpsOMGeometryFigureCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGlyphs(::windows::core::IUnknown);
impl IXpsOMGlyphs {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLocal)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUnicodeString)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlyphIndexCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGlyphIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlyphMappingCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGlyphMappings)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProhibitedCaretStopCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProhibitedCaretStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(prohibitedcaretstopcount), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBidiLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsSideways)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceFontName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStyleSimulations(&self) -> ::windows::core::Result<XPS_STYLE_SIMULATION> {
        let mut result__: XPS_STYLE_SIMULATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStyleSimulations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_STYLE_SIMULATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStyleSimulations)(::core::mem::transmute_copy(self), ::core::mem::transmute(stylesimulations)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOrigin)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOrigin)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFontRenderingEmSize(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFontRenderingEmSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFontRenderingEmSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontrenderingemsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFontResource(&self) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFontResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFontResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFontResource)(::core::mem::transmute_copy(self), fontresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFontFaceIndex(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFontFaceIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFontFaceIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontfaceindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, fillbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillBrushLocal)(::core::mem::transmute_copy(self), fillbrush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillBrushLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphsEditor(&self) -> ::windows::core::Result<IXpsOMGlyphsEditor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlyphsEditor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphsEditor>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
    }
}
impl ::core::convert::From<IXpsOMGlyphs> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGlyphs> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMGlyphs> for IXpsOMShareable {
    fn from(value: IXpsOMGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGlyphs> for IXpsOMShareable {
    fn from(value: &IXpsOMGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMGlyphs> for IXpsOMVisual {
    fn from(value: IXpsOMGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGlyphs> for IXpsOMVisual {
    fn from(value: &IXpsOMGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &'a IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGlyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGlyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGlyphs {}
impl ::core::fmt::Debug for IXpsOMGlyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGlyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGlyphs {
    type Vtable = IXpsOMGlyphs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphs_Vtbl {
    pub base: IXpsOMVisual_Vtbl,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub GetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT,
    pub GetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT,
    pub SetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphsEditor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGlyphsEditor(::windows::core::IUnknown);
impl IXpsOMGlyphsEditor {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn ApplyEdits(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyEdits)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUnicodeString)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetUnicodeString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, unicodestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUnicodeString)(::core::mem::transmute_copy(self), unicodestring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlyphIndexCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGlyphIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGlyphIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlyphMappingCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGlyphMappings)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGlyphMappings)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProhibitedCaretStopCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProhibitedCaretStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProhibitedCaretStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBidiLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBidiLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(bidilevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsSideways)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsSideways<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, issideways: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsSideways)(::core::mem::transmute_copy(self), issideways.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceFontName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDeviceFontName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, devicefontname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceFontName)(::core::mem::transmute_copy(self), devicefontname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMGlyphsEditor> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGlyphsEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGlyphsEditor> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGlyphsEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGlyphsEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGlyphsEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGlyphsEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGlyphsEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGlyphsEditor {}
impl ::core::fmt::Debug for IXpsOMGlyphsEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGlyphsEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGlyphsEditor {
    type Vtable = IXpsOMGlyphsEditor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsEditor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ApplyEdits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub SetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub SetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub SetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientBrush(::windows::core::IUnknown);
impl IXpsOMGradientBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGradientStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
}
impl ::core::convert::From<IXpsOMGradientBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGradientBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMGradientBrush> for IXpsOMShareable {
    fn from(value: IXpsOMGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGradientBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMGradientBrush> for IXpsOMBrush {
    fn from(value: IXpsOMGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGradientBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientBrush {}
impl ::core::fmt::Debug for IXpsOMGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGradientBrush {
    type Vtable = IXpsOMGradientBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedb59622_61a2_42c3_bace_acf2286c06bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientBrush_Vtbl {
    pub base: IXpsOMBrush_Vtbl,
    pub GetGradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientStop(::windows::core::IUnknown);
impl IXpsOMGradientStop {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOffset(&self, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColor<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
}
impl ::core::convert::From<IXpsOMGradientStop> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGradientStop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGradientStop> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGradientStop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGradientStop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGradientStop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientStop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientStop {}
impl ::core::fmt::Debug for IXpsOMGradientStop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientStop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGradientStop {
    type Vtable = IXpsOMGradientStop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStop_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientStopCollection(::windows::core::IUnknown);
impl IXpsOMGradientStopCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, stop: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), stop.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMGradientStopCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMGradientStopCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMGradientStopCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMGradientStopCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMGradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMGradientStopCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMGradientStopCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientStopCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientStopCollection {}
impl ::core::fmt::Debug for IXpsOMGradientStopCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientStopCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMGradientStopCollection {
    type Vtable = IXpsOMGradientStopCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageBrush(::windows::core::IUnknown);
impl IXpsOMImageBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetImageResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImageResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetImageResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetImageResource)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColorProfileResource(&self) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetColorProfileResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, colorprofileresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorProfileResource)(::core::mem::transmute_copy(self), colorprofileresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
    }
}
impl ::core::convert::From<IXpsOMImageBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMImageBrush> for IXpsOMShareable {
    fn from(value: IXpsOMImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMImageBrush> for IXpsOMBrush {
    fn from(value: IXpsOMImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMImageBrush> for IXpsOMTileBrush {
    fn from(value: IXpsOMImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageBrush> for IXpsOMTileBrush {
    fn from(value: &IXpsOMImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for &'a IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageBrush {}
impl ::core::fmt::Debug for IXpsOMImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMImageBrush {
    type Vtable = IXpsOMImageBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df0b466_d382_49ef_8550_dd94c80242e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrush_Vtbl {
    pub base: IXpsOMTileBrush_Vtbl,
    pub GetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageResource(::windows::core::IUnknown);
impl IXpsOMImageResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, imagetype: XPS_IMAGE_TYPE, partname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(imagetype), partname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetImageType(&self) -> ::windows::core::Result<XPS_IMAGE_TYPE> {
        let mut result__: XPS_IMAGE_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_IMAGE_TYPE>(result__)
    }
}
impl ::core::convert::From<IXpsOMImageResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMImageResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMImageResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMImageResource> for IXpsOMPart {
    fn from(value: IXpsOMImageResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageResource> for IXpsOMPart {
    fn from(value: &IXpsOMImageResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMImageResource> for IXpsOMResource {
    fn from(value: IXpsOMImageResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageResource> for IXpsOMResource {
    fn from(value: &IXpsOMImageResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMImageResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageResource {}
impl ::core::fmt::Debug for IXpsOMImageResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMImageResource {
    type Vtable = IXpsOMImageResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3db8417d_ae50_485e_9a44_d7758f78a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageResourceCollection(::windows::core::IUnknown);
impl IXpsOMImageResourceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByPartName)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMImageResourceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMImageResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMImageResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMImageResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMImageResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMImageResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMImageResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageResourceCollection {}
impl ::core::fmt::Debug for IXpsOMImageResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMImageResourceCollection {
    type Vtable = IXpsOMImageResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMLinearGradientBrush(::windows::core::IUnknown);
impl IXpsOMLinearGradientBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGradientStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStartPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetEndPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEndPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEndPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(endpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
}
impl ::core::convert::From<IXpsOMLinearGradientBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMLinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMLinearGradientBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMLinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMLinearGradientBrush> for IXpsOMShareable {
    fn from(value: IXpsOMLinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMLinearGradientBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMLinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMLinearGradientBrush> for IXpsOMBrush {
    fn from(value: IXpsOMLinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMLinearGradientBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMLinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMLinearGradientBrush> for IXpsOMGradientBrush {
    fn from(value: IXpsOMLinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMLinearGradientBrush> for IXpsOMGradientBrush {
    fn from(value: &IXpsOMLinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for &'a IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMLinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMLinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMLinearGradientBrush {}
impl ::core::fmt::Debug for IXpsOMLinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMLinearGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMLinearGradientBrush {
    type Vtable = IXpsOMLinearGradientBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005e279f_c30d_40ff_93ec_1950d3c528db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrush_Vtbl {
    pub base: IXpsOMGradientBrush_Vtbl,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMMatrixTransform(::windows::core::IUnknown);
impl IXpsOMMatrixTransform {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetMatrix(&self) -> ::windows::core::Result<XPS_MATRIX> {
        let mut result__: XPS_MATRIX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_MATRIX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
}
impl ::core::convert::From<IXpsOMMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: IXpsOMMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMMatrixTransform> for IXpsOMShareable {
    fn from(value: IXpsOMMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMMatrixTransform> for IXpsOMShareable {
    fn from(value: &IXpsOMMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMMatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMMatrixTransform {}
impl ::core::fmt::Debug for IXpsOMMatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMMatrixTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMMatrixTransform {
    type Vtable = IXpsOMMatrixTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransform_Vtbl {
    pub base: IXpsOMShareable_Vtbl,
    pub GetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMNameCollection(::windows::core::IUnknown);
impl IXpsOMNameCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXpsOMNameCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMNameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMNameCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMNameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMNameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMNameCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMNameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMNameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMNameCollection {}
impl ::core::fmt::Debug for IXpsOMNameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMNameCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMNameCollection {
    type Vtable = IXpsOMNameCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bddf8ec_c915_421b_a166_d173d25653d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMObjectFactory(::windows::core::IUnknown);
impl IXpsOMObjectFactory {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageFromFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageFromStream)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStoryFragmentsResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDocumentStructureResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSignatureBlockResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRemoteDictionaryResource)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePartResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDocumentSequence)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDocument)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePageReference)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePage)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePageFromStream)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCanvas)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGlyphs)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGeometryFigure)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSolidColorBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateColorProfileResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateImageBrush)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVisualBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateImageResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePrintTicketResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFontResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGradientStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearGradientBrush)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRadialGradientBrush)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCoreProperties)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePartUriCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        filename: Param0,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: Param3,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param5,
        coreproperties: Param6,
        packagethumbnail: Param7,
        documentsequenceprintticket: Param8,
        discardcontrolpartname: Param9,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageWriterOnFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param4: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param6: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param7: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        outputstream: Param0,
        optimizemarkupsize: Param1,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param3,
        coreproperties: Param4,
        packagethumbnail: Param5,
        documentsequenceprintticket: Param6,
        discardcontrolpartname: Param7,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageWriterOnStream)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, uri: Param0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePartUri)(::core::mem::transmute_copy(self), uri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, filename: Param0) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReadOnlyStreamOnFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
}
impl ::core::convert::From<IXpsOMObjectFactory> for ::windows::core::IUnknown {
    fn from(value: IXpsOMObjectFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMObjectFactory> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMObjectFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMObjectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMObjectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMObjectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMObjectFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMObjectFactory {}
impl ::core::fmt::Debug for IXpsOMObjectFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMObjectFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory {
    type Vtable = IXpsOMObjectFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateStoryFragmentsResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentStructureResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateSignatureBlockResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateSignatureBlockResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream: usize,
    pub CreatePartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentSequence: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocument: usize,
    pub CreatePageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows::core::PCWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream: usize,
    pub CreateCanvas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateGeometryFigure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateColorProfileResource: usize,
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateVisualBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateImageResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePrintTicketResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateFontResource: usize,
    pub CreateGradientStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateCoreProperties: usize,
    pub CreateDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePartUriCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamOnFile: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMObjectFactory1(::windows::core::IUnknown);
impl IXpsOMObjectFactory1 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePackage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePackageFromFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePackageFromStream)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStoryFragmentsResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDocumentStructureResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSignatureBlockResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRemoteDictionaryResource)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRemoteDictionaryResourceFromStream)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePartResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDocumentSequence)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDocument)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePageReference)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePage)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePageFromStream)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateCanvas)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGlyphs)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGeometryFigure)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSolidColorBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateColorProfileResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateImageBrush)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVisualBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateImageResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePrintTicketResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateFontResource)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGradientStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateLinearGradientBrush)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRadialGradientBrush)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateCoreProperties)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePartUriCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        filename: Param0,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: Param3,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param5,
        coreproperties: Param6,
        packagethumbnail: Param7,
        documentsequenceprintticket: Param8,
        discardcontrolpartname: Param9,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePackageWriterOnFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param4: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param6: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param7: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        outputstream: Param0,
        optimizemarkupsize: Param1,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param3,
        coreproperties: Param4,
        packagethumbnail: Param5,
        documentsequenceprintticket: Param6,
        discardcontrolpartname: Param7,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePackageWriterOnStream)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, uri: Param0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePartUri)(::core::mem::transmute_copy(self), uri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, filename: Param0) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateReadOnlyStreamOnFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentTypeFromFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, filename: Param0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentTypeFromFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, xpsdocumentstream: Param0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentTypeFromStream)(::core::mem::transmute_copy(self), xpsdocumentstream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn ConvertHDPhotoToJpegXR<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertHDPhotoToJpegXR)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn ConvertJpegXRToHDPhoto<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertJpegXRToHDPhoto)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        filename: Param0,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: Param3,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param5,
        coreproperties: Param6,
        packagethumbnail: Param7,
        documentsequenceprintticket: Param8,
        discardcontrolpartname: Param9,
        documenttype: XPS_DOCUMENT_TYPE,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageWriterOnFile1)(
            ::core::mem::transmute_copy(self),
            filename.into_param().abi(),
            ::core::mem::transmute(securityattributes),
            ::core::mem::transmute(flagsandattributes),
            optimizemarkupsize.into_param().abi(),
            ::core::mem::transmute(interleaving),
            documentsequencepartname.into_param().abi(),
            coreproperties.into_param().abi(),
            packagethumbnail.into_param().abi(),
            documentsequenceprintticket.into_param().abi(),
            discardcontrolpartname.into_param().abi(),
            ::core::mem::transmute(documenttype),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param4: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param6: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param7: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
        &self,
        outputstream: Param0,
        optimizemarkupsize: Param1,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param3,
        coreproperties: Param4,
        packagethumbnail: Param5,
        documentsequenceprintticket: Param6,
        discardcontrolpartname: Param7,
        documenttype: XPS_DOCUMENT_TYPE,
    ) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageWriterOnStream1)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(documenttype), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreatePackage1(&self) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackage1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageFromStream1)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePackageFromFile1)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage1<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePage1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePageFromStream1)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, parturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream1)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMObjectFactory1> for ::windows::core::IUnknown {
    fn from(value: IXpsOMObjectFactory1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMObjectFactory1> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMObjectFactory1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMObjectFactory1> for IXpsOMObjectFactory {
    fn from(value: IXpsOMObjectFactory1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMObjectFactory1> for IXpsOMObjectFactory {
    fn from(value: &IXpsOMObjectFactory1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMObjectFactory> for IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMObjectFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMObjectFactory> for &'a IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMObjectFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMObjectFactory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMObjectFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMObjectFactory1 {}
impl ::core::fmt::Debug for IXpsOMObjectFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMObjectFactory1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory1 {
    type Vtable = IXpsOMObjectFactory1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory1_Vtbl {
    pub base: IXpsOMObjectFactory_Vtbl,
    pub GetDocumentTypeFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentTypeFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentTypeFromStream: usize,
    pub ConvertHDPhotoToJpegXR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ConvertJpegXRToHDPhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows::core::PCWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackage(::windows::core::IUnknown);
impl IXpsOMPackage {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentSequence)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentSequence)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCoreProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoreProperties)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDiscardControlPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDiscardControlPartName)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetThumbnailResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetThumbnailResource)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToStream)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPackage> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackage> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackage {}
impl ::core::fmt::Debug for IXpsOMPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPackage {
    type Vtable = IXpsOMPackage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18c3df65_81e1_4674_91dc_fc452f5a416f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDiscardControlPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetDiscardControlPartName: usize,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackage1(::windows::core::IUnknown);
impl IXpsOMPackage1 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDocumentSequence)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDocumentSequence)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCoreProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCoreProperties)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDiscardControlPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDiscardControlPartName)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetThumbnailResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetThumbnailResource)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WriteToFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WriteToStream)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToFile1)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, outputstream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToStream1)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
impl ::core::convert::From<IXpsOMPackage1> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPackage1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackage1> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPackage1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPackage1> for IXpsOMPackage {
    fn from(value: IXpsOMPackage1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackage1> for IXpsOMPackage {
    fn from(value: &IXpsOMPackage1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackage> for IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackage> for &'a IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPackage1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackage1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackage1 {}
impl ::core::fmt::Debug for IXpsOMPackage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackage1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPackage1 {
    type Vtable = IXpsOMPackage1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1_Vtbl {
    pub base: IXpsOMPackage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageTarget(::windows::core::IUnknown);
impl IXpsOMPackageTarget {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateXpsOMPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, documentsequenceprintticket: Param1, discardcontrolpartname: Param2) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateXpsOMPackageWriter)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
}
impl ::core::convert::From<IXpsOMPackageTarget> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPackageTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackageTarget> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPackageTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageTarget {}
impl ::core::fmt::Debug for IXpsOMPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPackageTarget {
    type Vtable = IXpsOMPackageTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTarget_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageWriter(::windows::core::IUnknown);
impl IXpsOMPackageWriter {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>>(&self, documentpartname: Param0, documentprintticket: Param1, documentstructure: Param2, signatureblockresources: Param3, restrictedfonts: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartNewDocument)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, page: Param0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: Param2, storyfragments: Param3, pageprintticket: Param4, pagethumbnail: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPage)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddResource)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsClosed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IXpsOMPackageWriter> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPackageWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackageWriter> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPackageWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPackageWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageWriter {}
impl ::core::fmt::Debug for IXpsOMPackageWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter {
    type Vtable = IXpsOMPackageWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageWriter3D(::windows::core::IUnknown);
impl IXpsOMPackageWriter3D {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>>(&self, documentpartname: Param0, documentprintticket: Param1, documentstructure: Param2, signatureblockresources: Param3, restrictedfonts: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartNewDocument)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, page: Param0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: Param2, storyfragments: Param3, pageprintticket: Param4, pagethumbnail: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddPage)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddResource)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsClosed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddModelTexture<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, texturepartname: Param0, texturedata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddModelTexture)(::core::mem::transmute_copy(self), texturepartname.into_param().abi(), texturedata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, printticketpartname: Param0, printticketdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModelPrintTicket)(::core::mem::transmute_copy(self), printticketpartname.into_param().abi(), printticketdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPackageWriter3D> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPackageWriter3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackageWriter3D> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPackageWriter3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPackageWriter3D> for IXpsOMPackageWriter {
    fn from(value: IXpsOMPackageWriter3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPackageWriter3D> for IXpsOMPackageWriter {
    fn from(value: &IXpsOMPackageWriter3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackageWriter> for IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackageWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackageWriter> for &'a IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackageWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPackageWriter3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageWriter3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageWriter3D {}
impl ::core::fmt::Debug for IXpsOMPackageWriter3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageWriter3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter3D {
    type Vtable = IXpsOMPackageWriter3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter3D_Vtbl {
    pub base: IXpsOMPackageWriter_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddModelTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddModelTexture: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetModelPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetModelPrintTicket: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPage(::windows::core::IUnknown);
impl IXpsOMPage {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisuals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContentBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContentBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBleedBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBleedBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionaryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictionaryLocal)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionaryResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictionaryResource)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateUnusedLookupKey)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
}
impl ::core::convert::From<IXpsOMPage> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPage> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPage> for IXpsOMPart {
    fn from(value: IXpsOMPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPage> for IXpsOMPart {
    fn from(value: &IXpsOMPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPage {}
impl ::core::fmt::Debug for IXpsOMPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPage {
    type Vtable = IXpsOMPage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e18888_f120_4fee_8c68_35296eae91d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage_Vtbl {
    pub base: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPage1(::windows::core::IUnknown);
impl IXpsOMPage1 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVisuals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetContentBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetContentBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBleedBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBleedBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDictionaryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDictionaryLocal)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDictionaryResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDictionaryResource)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Write)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GenerateUnusedLookupKey)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write1)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
impl ::core::convert::From<IXpsOMPage1> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPage1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPage1> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPage1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPage1> for IXpsOMPart {
    fn from(value: IXpsOMPage1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPage1> for IXpsOMPart {
    fn from(value: &IXpsOMPage1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPage1> for IXpsOMPage {
    fn from(value: IXpsOMPage1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPage1> for IXpsOMPage {
    fn from(value: &IXpsOMPage1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPage> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPage> for &'a IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPage1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPage1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPage1 {}
impl ::core::fmt::Debug for IXpsOMPage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPage1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPage1 {
    type Vtable = IXpsOMPage1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x305b60ef_6892_4dda_9cbb_3aa65974508a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1_Vtbl {
    pub base: IXpsOMPage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPageReference(::windows::core::IUnknown);
impl IXpsOMPageReference {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPage(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>>(&self, page: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPage)(::core::mem::transmute_copy(self), page.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn DiscardPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DiscardPage)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPageLoaded(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsPageLoaded)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAdvisoryPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAdvisoryPageDimensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStoryFragmentsResource(&self) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStoryFragmentsResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>>(&self, storyfragmentsresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStoryFragmentsResource)(::core::mem::transmute_copy(self), storyfragmentsresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPrintTicketResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrintTicketResource)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetThumbnailResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetThumbnailResource)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CollectLinkTargets(&self) -> ::windows::core::Result<IXpsOMNameCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CollectLinkTargets)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMNameCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CollectPartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CollectPartResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasRestrictedFonts(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasRestrictedFonts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
}
impl ::core::convert::From<IXpsOMPageReference> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPageReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPageReference> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPageReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPageReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPageReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPageReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPageReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPageReference {}
impl ::core::fmt::Debug for IXpsOMPageReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPageReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPageReference {
    type Vtable = IXpsOMPageReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed360180_6f92_4998_890d_2f208531a0a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReference_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPageLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPageLoaded: usize,
    pub GetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CollectLinkTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CollectPartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasRestrictedFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasRestrictedFonts: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPageReferenceCollection(::windows::core::IUnknown);
impl IXpsOMPageReferenceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, pagereference: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), pagereference.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPageReferenceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPageReferenceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPageReferenceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPageReferenceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPageReferenceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPageReferenceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPageReferenceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPageReferenceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPageReferenceCollection {}
impl ::core::fmt::Debug for IXpsOMPageReferenceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPageReferenceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPageReferenceCollection {
    type Vtable = IXpsOMPageReferenceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca16ba4d_e7b9_45c5_958b_f98022473745);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPart(::windows::core::IUnknown);
impl IXpsOMPart {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPart> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPart> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPart {}
impl ::core::fmt::Debug for IXpsOMPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPart").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPart {
    type Vtable = IXpsOMPart_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPart_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPartResources(::windows::core::IUnknown);
impl IXpsOMPartResources {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFontResources(&self) -> ::windows::core::Result<IXpsOMFontResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFontResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResourceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetImageResources(&self) -> ::windows::core::Result<IXpsOMImageResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImageResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResourceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColorProfileResources(&self) -> ::windows::core::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetColorProfileResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResourceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRemoteDictionaryResources(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRemoteDictionaryResources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResourceCollection>(result__)
    }
}
impl ::core::convert::From<IXpsOMPartResources> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPartResources) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPartResources> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPartResources) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPartResources {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPartResources {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPartResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPartResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPartResources {}
impl ::core::fmt::Debug for IXpsOMPartResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPartResources").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPartResources {
    type Vtable = IXpsOMPartResources_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4cf7729_4864_4275_99b3_a8717163ecaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResources_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFontResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPartUriCollection(::windows::core::IUnknown);
impl IXpsOMPartUriCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPartUriCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPartUriCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPartUriCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPartUriCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPartUriCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPartUriCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPartUriCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPartUriCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPartUriCollection {}
impl ::core::fmt::Debug for IXpsOMPartUriCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPartUriCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPartUriCollection {
    type Vtable = IXpsOMPartUriCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57c650d4_067c_4893_8c33_f62a0633730f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartUriCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    InsertAt: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    Append: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPath(::windows::core::IUnknown);
impl IXpsOMPath {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLocal)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClipGeometryLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipGeometryLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGeometryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, geometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGeometryLocal)(::core::mem::transmute_copy(self), geometry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGeometryLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGeometryLookup)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessibilityShortDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, shortdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccessibilityShortDescription)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessibilityLongDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, longdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccessibilityLongDescription)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapsToPixels(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSnapsToPixels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSnapsToPixels<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, snapstopixels: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapsToPixels)(::core::mem::transmute_copy(self), snapstopixels.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeBrushLocal)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeBrushLookup)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeDashes(&self) -> ::windows::core::Result<IXpsOMDashCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeDashes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDashCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeDashCap(&self) -> ::windows::core::Result<XPS_DASH_CAP> {
        let mut result__: XPS_DASH_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeDashCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DASH_CAP>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeDashCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashcap)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeDashOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeDashOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeDashOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeStartLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__: XPS_LINE_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeStartLineCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_CAP>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeStartLineCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokestartlinecap)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeEndLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__: XPS_LINE_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeEndLineCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_CAP>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeEndLineCap)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokeendlinecap)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeLineJoin(&self) -> ::windows::core::Result<XPS_LINE_JOIN> {
        let mut result__: XPS_LINE_JOIN = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeLineJoin)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_JOIN>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeLineJoin)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokelinejoin)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeMiterLimit(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeMiterLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeMiterLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokemiterlimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetStrokeThickness(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStrokeThickness)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrokeThickness)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokethickness)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillBrushLocal)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFillBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillBrushLookup)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
    }
}
impl ::core::convert::From<IXpsOMPath> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPath> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPath> for IXpsOMShareable {
    fn from(value: IXpsOMPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPath> for IXpsOMShareable {
    fn from(value: &IXpsOMPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPath> for IXpsOMVisual {
    fn from(value: IXpsOMPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPath> for IXpsOMVisual {
    fn from(value: &IXpsOMPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &'a IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPath {}
impl ::core::fmt::Debug for IXpsOMPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPath {
    type Vtable = IXpsOMPath_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37d38bb6_3ee9_4110_9312_14b194163337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPath_Vtbl {
    pub base: IXpsOMVisual_Vtbl,
    pub GetGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapsToPixels: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSnapsToPixels: usize,
    pub GetStrokeBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStrokeDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT,
    pub GetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub GetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT,
    pub GetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPrintTicketResource(::windows::core::IUnknown);
impl IXpsOMPrintTicketResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMPrintTicketResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMPrintTicketResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPrintTicketResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMPrintTicketResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPrintTicketResource> for IXpsOMPart {
    fn from(value: IXpsOMPrintTicketResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPrintTicketResource> for IXpsOMPart {
    fn from(value: &IXpsOMPrintTicketResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMPrintTicketResource> for IXpsOMResource {
    fn from(value: IXpsOMPrintTicketResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMPrintTicketResource> for IXpsOMResource {
    fn from(value: &IXpsOMPrintTicketResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMPrintTicketResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMPrintTicketResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPrintTicketResource {}
impl ::core::fmt::Debug for IXpsOMPrintTicketResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPrintTicketResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMPrintTicketResource {
    type Vtable = IXpsOMPrintTicketResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPrintTicketResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRadialGradientBrush(::windows::core::IUnknown);
impl IXpsOMRadialGradientBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGradientStops)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSpreadMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetColorInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCenter(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCenter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenter)(::core::mem::transmute_copy(self), ::core::mem::transmute(center)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRadiiSizes(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRadiiSizes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRadiiSizes)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiisizes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetGradientOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGradientOrigin)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGradientOrigin)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
}
impl ::core::convert::From<IXpsOMRadialGradientBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMRadialGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRadialGradientBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMRadialGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRadialGradientBrush> for IXpsOMShareable {
    fn from(value: IXpsOMRadialGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRadialGradientBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMRadialGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRadialGradientBrush> for IXpsOMBrush {
    fn from(value: IXpsOMRadialGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRadialGradientBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMRadialGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRadialGradientBrush> for IXpsOMGradientBrush {
    fn from(value: IXpsOMRadialGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRadialGradientBrush> for IXpsOMGradientBrush {
    fn from(value: &IXpsOMRadialGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for &'a IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMRadialGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMRadialGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRadialGradientBrush {}
impl ::core::fmt::Debug for IXpsOMRadialGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRadialGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMRadialGradientBrush {
    type Vtable = IXpsOMRadialGradientBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f207e5_08bf_413c_96b1_b82b4064176b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRadialGradientBrush_Vtbl {
    pub base: IXpsOMGradientBrush_Vtbl,
    pub GetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionary<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictionary)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource> for IXpsOMPart {
    fn from(value: IXpsOMRemoteDictionaryResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource> for IXpsOMPart {
    fn from(value: &IXpsOMRemoteDictionaryResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource> for IXpsOMResource {
    fn from(value: IXpsOMRemoteDictionaryResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource> for IXpsOMResource {
    fn from(value: &IXpsOMRemoteDictionaryResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResource {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource {
    type Vtable = IXpsOMRemoteDictionaryResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource1(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource1 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDictionary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDictionary<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDictionary)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>>(&self, stream: Param0, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write1)(::core::mem::transmute_copy(self), stream.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource1> for ::windows::core::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource1> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource1> for IXpsOMPart {
    fn from(value: IXpsOMRemoteDictionaryResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource1> for IXpsOMPart {
    fn from(value: &IXpsOMRemoteDictionaryResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource1> for IXpsOMResource {
    fn from(value: IXpsOMRemoteDictionaryResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource1> for IXpsOMResource {
    fn from(value: &IXpsOMRemoteDictionaryResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource1> for IXpsOMRemoteDictionaryResource {
    fn from(value: IXpsOMRemoteDictionaryResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource1> for IXpsOMRemoteDictionaryResource {
    fn from(value: &IXpsOMRemoteDictionaryResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMRemoteDictionaryResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource> for &'a IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMRemoteDictionaryResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResource1 {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResource1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource1 {
    type Vtable = IXpsOMRemoteDictionaryResource1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1_Vtbl {
    pub base: IXpsOMRemoteDictionaryResource_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResourceCollection(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByPartName)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResourceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMRemoteDictionaryResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMRemoteDictionaryResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResourceCollection {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResourceCollection {
    type Vtable = IXpsOMRemoteDictionaryResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c38db61_7fec_464a_87bd_41e3bef018be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMResource(::windows::core::IUnknown);
impl IXpsOMResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMResource> for IXpsOMPart {
    fn from(value: IXpsOMResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMResource> for IXpsOMPart {
    fn from(value: &IXpsOMResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMResource {}
impl ::core::fmt::Debug for IXpsOMResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMResource {
    type Vtable = IXpsOMResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResource_Vtbl {
    pub base: IXpsOMPart_Vtbl,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMShareable(::windows::core::IUnknown);
impl IXpsOMShareable {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
}
impl ::core::convert::From<IXpsOMShareable> for ::windows::core::IUnknown {
    fn from(value: IXpsOMShareable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMShareable> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMShareable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMShareable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMShareable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMShareable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMShareable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMShareable {}
impl ::core::fmt::Debug for IXpsOMShareable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMShareable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMShareable {
    type Vtable = IXpsOMShareable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareable_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResource(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMSignatureBlockResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMSignatureBlockResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMSignatureBlockResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMSignatureBlockResource> for IXpsOMPart {
    fn from(value: IXpsOMSignatureBlockResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResource> for IXpsOMPart {
    fn from(value: &IXpsOMSignatureBlockResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMSignatureBlockResource> for IXpsOMResource {
    fn from(value: IXpsOMSignatureBlockResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResource> for IXpsOMResource {
    fn from(value: &IXpsOMSignatureBlockResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMSignatureBlockResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMSignatureBlockResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSignatureBlockResource {}
impl ::core::fmt::Debug for IXpsOMSignatureBlockResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSignatureBlockResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResource {
    type Vtable = IXpsOMSignatureBlockResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4776ad35_2e04_4357_8743_ebf6c171a905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResourceCollection(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, signatureblockresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetByPartName)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMSignatureBlockResourceCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMSignatureBlockResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMSignatureBlockResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMSignatureBlockResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMSignatureBlockResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMSignatureBlockResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMSignatureBlockResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSignatureBlockResourceCollection {}
impl ::core::fmt::Debug for IXpsOMSignatureBlockResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSignatureBlockResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResourceCollection {
    type Vtable = IXpsOMSignatureBlockResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSolidColorBrush(::windows::core::IUnknown);
impl IXpsOMSolidColorBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetColor<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
}
impl ::core::convert::From<IXpsOMSolidColorBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMSolidColorBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSolidColorBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMSolidColorBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMSolidColorBrush> for IXpsOMShareable {
    fn from(value: IXpsOMSolidColorBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSolidColorBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMSolidColorBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMSolidColorBrush> for IXpsOMBrush {
    fn from(value: IXpsOMSolidColorBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMSolidColorBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMSolidColorBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMSolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMSolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSolidColorBrush {}
impl ::core::fmt::Debug for IXpsOMSolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSolidColorBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMSolidColorBrush {
    type Vtable = IXpsOMSolidColorBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06f9f05_3be9_4763_98a8_094fc672e488);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrush_Vtbl {
    pub base: IXpsOMBrush_Vtbl,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMStoryFragmentsResource(::windows::core::IUnknown);
impl IXpsOMStoryFragmentsResource {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPartName)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMStoryFragmentsResource> for ::windows::core::IUnknown {
    fn from(value: IXpsOMStoryFragmentsResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMStoryFragmentsResource> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMStoryFragmentsResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMStoryFragmentsResource> for IXpsOMPart {
    fn from(value: IXpsOMStoryFragmentsResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMStoryFragmentsResource> for IXpsOMPart {
    fn from(value: &IXpsOMStoryFragmentsResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &'a IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMStoryFragmentsResource> for IXpsOMResource {
    fn from(value: IXpsOMStoryFragmentsResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMStoryFragmentsResource> for IXpsOMResource {
    fn from(value: &IXpsOMStoryFragmentsResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &'a IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMStoryFragmentsResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMStoryFragmentsResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMStoryFragmentsResource {}
impl ::core::fmt::Debug for IXpsOMStoryFragmentsResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMStoryFragmentsResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMStoryFragmentsResource {
    type Vtable = IXpsOMStoryFragmentsResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b3ca09_0473_4282_87ae_1780863223f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMStoryFragmentsResource_Vtbl {
    pub base: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMThumbnailGenerator(::windows::core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateThumbnail<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param3: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, page: Param0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: Param3) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateThumbnail)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(thumbnailtype), ::core::mem::transmute(thumbnailsize), imageresourcepartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
}
impl ::core::convert::From<IXpsOMThumbnailGenerator> for ::windows::core::IUnknown {
    fn from(value: IXpsOMThumbnailGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMThumbnailGenerator> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMThumbnailGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMThumbnailGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMThumbnailGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMThumbnailGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMThumbnailGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMThumbnailGenerator {}
impl ::core::fmt::Debug for IXpsOMThumbnailGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMThumbnailGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMThumbnailGenerator {
    type Vtable = IXpsOMThumbnailGenerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15b873d5_1971_41e8_83a3_6578403064c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGenerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMTileBrush(::windows::core::IUnknown);
impl IXpsOMTileBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
}
impl ::core::convert::From<IXpsOMTileBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMTileBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMTileBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMTileBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMTileBrush> for IXpsOMShareable {
    fn from(value: IXpsOMTileBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMTileBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMTileBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMTileBrush> for IXpsOMBrush {
    fn from(value: IXpsOMTileBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMTileBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMTileBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMTileBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMTileBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMTileBrush {}
impl ::core::fmt::Debug for IXpsOMTileBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMTileBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMTileBrush {
    type Vtable = IXpsOMTileBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fc2328d_d722_4a54_b2ec_be90218a789e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMTileBrush_Vtbl {
    pub base: IXpsOMBrush_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub SetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisual(::windows::core::IUnknown);
impl IXpsOMVisual {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLocal)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClipGeometry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClipGeometryLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClipGeometryLocal)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClipGeometryLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClipGeometryLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOpacityMaskBrush)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacityMaskBrushLocal)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacityMaskBrushLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsHyperlinkTarget)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHyperlinkNavigateUri)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguage)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMVisual> for ::windows::core::IUnknown {
    fn from(value: IXpsOMVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisual> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMVisual> for IXpsOMShareable {
    fn from(value: IXpsOMVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisual> for IXpsOMShareable {
    fn from(value: &IXpsOMVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisual {}
impl ::core::fmt::Debug for IXpsOMVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMVisual {
    type Vtable = IXpsOMVisual_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisual_Vtbl {
    pub base: IXpsOMShareable_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetClipGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHyperlinkNavigateUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHyperlinkNavigateUri: usize,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisualBrush(::windows::core::IUnknown);
impl IXpsOMVisualBrush {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetOwner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOpacity)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLocal)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransformLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformLookup)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTileMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisual(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisual)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisualLocal(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisualLocal)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetVisualLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVisualLocal)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetVisualLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVisualLookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetVisualLookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVisualLookup)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
    }
}
impl ::core::convert::From<IXpsOMVisualBrush> for ::windows::core::IUnknown {
    fn from(value: IXpsOMVisualBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisualBrush> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMVisualBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMVisualBrush> for IXpsOMShareable {
    fn from(value: IXpsOMVisualBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisualBrush> for IXpsOMShareable {
    fn from(value: &IXpsOMVisualBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &'a IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMVisualBrush> for IXpsOMBrush {
    fn from(value: IXpsOMVisualBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisualBrush> for IXpsOMBrush {
    fn from(value: &IXpsOMVisualBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &'a IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXpsOMVisualBrush> for IXpsOMTileBrush {
    fn from(value: IXpsOMVisualBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisualBrush> for IXpsOMTileBrush {
    fn from(value: &IXpsOMVisualBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for &'a IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMVisualBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisualBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisualBrush {}
impl ::core::fmt::Debug for IXpsOMVisualBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisualBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMVisualBrush {
    type Vtable = IXpsOMVisualBrush_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97e294af_5b37_46b4_8057_874d2f64119b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrush_Vtbl {
    pub base: IXpsOMTileBrush_Vtbl,
    pub GetVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisualCollection(::windows::core::IUnknown);
impl IXpsOMVisualCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsOMVisualCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsOMVisualCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsOMVisualCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsOMVisualCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsOMVisualCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsOMVisualCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsOMVisualCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisualCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisualCollection {}
impl ::core::fmt::Debug for IXpsOMVisualCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisualCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsOMVisualCollection {
    type Vtable = IXpsOMVisualCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignature(::windows::core::IUnknown);
impl IXpsSignature {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignatureValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturehashvalue), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCertificateEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcCertificateEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSigningTime(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSigningTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSigningTimeFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignaturePartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<XPS_SIGNATURE_STATUS> {
        let mut result__: XPS_SIGNATURE_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Verify)(::core::mem::transmute_copy(self), ::core::mem::transmute(x509certificate), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGNATURE_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__: XPS_SIGN_POLICY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomObjectEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomReferenceEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignatureXml)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSignatureXml(&self, signaturexml: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignatureXml)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(signaturexml)), signaturexml.len() as _).ok()
    }
}
impl ::core::convert::From<IXpsSignature> for ::windows::core::IUnknown {
    fn from(value: IXpsSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignature> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignature {}
impl ::core::fmt::Debug for IXpsSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignature {
    type Vtable = IXpsSignature_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ae4c93e_1ade_42fb_898b_3a5658284857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignature_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateEnumerator: usize,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Verify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Verify: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjectEnumerator: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferenceEnumerator: usize,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub SetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureBlock(::windows::core::IUnknown);
impl IXpsSignatureBlock {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRequests(&self) -> ::windows::core::Result<IXpsSignatureRequestCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRequests)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequestCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDocumentIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocumentName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateRequest<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, requestid: Param0) -> ::windows::core::Result<IXpsSignatureRequest> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRequest)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequest>(result__)
    }
}
impl ::core::convert::From<IXpsSignatureBlock> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureBlock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureBlock> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureBlock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureBlock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureBlock {}
impl ::core::fmt::Debug for IXpsSignatureBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureBlock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureBlock {
    type Vtable = IXpsSignatureBlock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151fac09_0b97_4ac6_a323_5e4297d4322b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlock_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    pub GetDocumentIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDocumentName: usize,
    pub CreateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::PCWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureBlockCollection(::windows::core::IUnknown);
impl IXpsSignatureBlockCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureBlock> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlock>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
impl ::core::convert::From<IXpsSignatureBlockCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureBlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureBlockCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureBlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureBlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureBlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureBlockCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureBlockCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureBlockCollection {}
impl ::core::fmt::Debug for IXpsSignatureBlockCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureBlockCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureBlockCollection {
    type Vtable = IXpsSignatureBlockCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23397050_fe99_467a_8dce_9237f074ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureCollection(::windows::core::IUnknown);
impl IXpsSignatureCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
impl ::core::convert::From<IXpsSignatureCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureCollection {}
impl ::core::fmt::Debug for IXpsSignatureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureCollection {
    type Vtable = IXpsSignatureCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureManager(::windows::core::IUnknown);
impl IXpsSignatureManager {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn LoadPackageFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadPackageFile)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadPackageStream)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<'a, Param0: ::windows::core::IntoParam<'a, IXpsSigningOptions>>(&self, signoptions: Param0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Sign)(::core::mem::transmute_copy(self), signoptions.into_param().abi(), ::core::mem::transmute(x509certificate), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureOriginPartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignatureOriginPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signatureoriginpartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignatureOriginPartName)(::core::mem::transmute_copy(self), signatureoriginpartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatures(&self) -> ::windows::core::Result<IXpsSignatureCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatures)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddSignatureBlock<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0, fixeddocumentindex: u32) -> ::windows::core::Result<IXpsSignatureBlock> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddSignatureBlock)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(fixeddocumentindex), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlock>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureBlocks(&self) -> ::windows::core::Result<IXpsSignatureBlockCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureBlocks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlockCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::core::Result<IXpsSigningOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSigningOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSigningOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn SavePackageToFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SavePackageToFile)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SavePackageToStream)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXpsSignatureManager> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureManager> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureManager {}
impl ::core::fmt::Debug for IXpsSignatureManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureManager {
    type Vtable = IXpsSignatureManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub LoadPackageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadPackageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadPackageStream: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignatureOriginPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignatureOriginPartName: usize,
    pub GetSignatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddSignatureBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddSignatureBlock: usize,
    pub GetSignatureBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub SavePackageToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    SavePackageToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SavePackageToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SavePackageToStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureRequest(::windows::core::IUnknown);
impl IXpsSignatureRequest {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetIntent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIntent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetIntent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, intent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntent)(::core::mem::transmute_copy(self), intent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRequestedSigner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRequestedSigner)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetRequestedSigner<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, signername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRequestedSigner)(::core::mem::transmute_copy(self), signername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRequestSignByDate(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRequestSignByDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetRequestSignByDate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, datestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRequestSignByDate)(::core::mem::transmute_copy(self), datestring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSigningLocale(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSigningLocale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSigningLocale<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, place: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSigningLocale)(::core::mem::transmute_copy(self), place.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpotLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(pagepartname), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSpotLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetRequestId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRequestId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignature)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
    }
}
impl ::core::convert::From<IXpsSignatureRequest> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureRequest> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureRequest {}
impl ::core::fmt::Debug for IXpsSignatureRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureRequest {
    type Vtable = IXpsSignatureRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequest_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSpotLocation: usize,
    pub SetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureRequestCollection(::windows::core::IUnknown);
impl IXpsSignatureRequestCollection {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureRequest> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequest>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
impl ::core::convert::From<IXpsSignatureRequestCollection> for ::windows::core::IUnknown {
    fn from(value: IXpsSignatureRequestCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSignatureRequestCollection> for ::windows::core::IUnknown {
    fn from(value: &IXpsSignatureRequestCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSignatureRequestCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSignatureRequestCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSignatureRequestCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureRequestCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureRequestCollection {}
impl ::core::fmt::Debug for IXpsSignatureRequestCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureRequestCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSignatureRequestCollection {
    type Vtable = IXpsSignatureRequestCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSigningOptions(::windows::core::IUnknown);
impl IXpsSigningOptions {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSignatureId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, signatureid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignatureId)(::core::mem::transmute_copy(self), signatureid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignatureMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetSignatureMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, signaturemethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignatureMethod)(::core::mem::transmute_copy(self), signaturemethod.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDigestMethod)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetDigestMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, digestmethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDigestMethod)(::core::mem::transmute_copy(self), digestmethod.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignaturePartName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignaturePartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignaturePartName)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__: XPS_SIGN_POLICY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSigningTimeFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSigningTimeFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomReferences)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCertificateSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcCertificateSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<XPS_SIGN_FLAGS> {
        let mut result__: XPS_SIGN_FLAGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_FLAGS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IXpsSigningOptions> for ::windows::core::IUnknown {
    fn from(value: IXpsSigningOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsSigningOptions> for ::windows::core::IUnknown {
    fn from(value: &IXpsSigningOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsSigningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsSigningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsSigningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsSigningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSigningOptions {}
impl ::core::fmt::Debug for IXpsSigningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSigningOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXpsSigningOptions {
    type Vtable = IXpsSigningOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7718eae4_3215_49be_af5b_594fef7fcfa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSigningOptions_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignaturePartName: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    pub SetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub SetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    SetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjects: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferences: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateSet: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
impl ::core::marker::Copy for PRINT_WINDOW_FLAGS {}
impl ::core::clone::Clone for PRINT_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRINT_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRINT_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRINT_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINT_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl ::core::marker::Copy for PSFEATURE_CUSTPAPER {}
impl ::core::clone::Clone for PSFEATURE_CUSTPAPER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSFEATURE_CUSTPAPER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_CUSTPAPER").field("lOrientation", &self.lOrientation).field("lWidth", &self.lWidth).field("lHeight", &self.lHeight).field("lWidthOffset", &self.lWidthOffset).field("lHeightOffset", &self.lHeightOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for PSFEATURE_CUSTPAPER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSFEATURE_CUSTPAPER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSFEATURE_CUSTPAPER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSFEATURE_CUSTPAPER {}
impl ::core::default::Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSFEATURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSFEATURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_OUTPUT").field("bPageIndependent", &self.bPageIndependent).field("bSetPageDevice", &self.bSetPageDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PSFEATURE_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSFEATURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSFEATURE_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl ::core::marker::Copy for PSINJECTDATA {}
impl ::core::clone::Clone for PSINJECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSINJECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSINJECTDATA").field("DataBytes", &self.DataBytes).field("InjectionPoint", &self.InjectionPoint).field("PageNumber", &self.PageNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for PSINJECTDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSINJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSINJECTDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSINJECTDATA {}
impl ::core::default::Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PSINJECT_POINT(pub u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
impl ::core::marker::Copy for PSINJECT_POINT {}
impl ::core::clone::Clone for PSINJECT_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSINJECT_POINT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PSINJECT_POINT {
    type Abi = Self;
}
impl ::core::fmt::Debug for PSINJECT_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSINJECT_POINT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSINJECT_POINT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSINJECT_POINT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSINJECT_POINT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSINJECT_POINT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSINJECT_POINT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdcblt: Param1, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintWindow(hwnd: super::super::Foundation::HWND, hdcblt: super::super::Graphics::Gdi::HDC, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PrintWindow(hwnd.into_param().abi(), hdcblt.into_param().abi(), ::core::mem::transmute(nflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetAbortProc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, proc: ABORTPROC) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAbortProc(hdc: super::super::Graphics::Gdi::HDC, proc: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(SetAbortProc(hdc.into_param().abi(), ::core::mem::transmute(proc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpdi: *const DOCINFOA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartDocA(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOA) -> i32;
        }
        ::core::mem::transmute(StartDocA(hdc.into_param().abi(), ::core::mem::transmute(lpdi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpdi: *const DOCINFOW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartDocW(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOW) -> i32;
        }
        ::core::mem::transmute(StartDocW(hdc.into_param().abi(), ::core::mem::transmute(lpdi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
        }
        ::core::mem::transmute(StartPage(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl ::core::marker::Copy for XPS_COLOR {}
impl ::core::clone::Clone for XPS_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_COLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_COLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_COLOR {}
impl ::core::default::Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl ::core::marker::Copy for XPS_COLOR_0 {}
impl ::core::clone::Clone for XPS_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_COLOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0 {}
impl ::core::default::Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl ::core::marker::Copy for XPS_COLOR_0_0 {}
impl ::core::clone::Clone for XPS_COLOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_0").field("channelCount", &self.channelCount).field("channels", &self.channels).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_COLOR_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_0 {}
impl ::core::default::Default for XPS_COLOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl ::core::marker::Copy for XPS_COLOR_0_1 {}
impl ::core::clone::Clone for XPS_COLOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_1").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_COLOR_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_1 {}
impl ::core::default::Default for XPS_COLOR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl ::core::marker::Copy for XPS_COLOR_0_2 {}
impl ::core::clone::Clone for XPS_COLOR_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_2").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_COLOR_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_2 {}
impl ::core::default::Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
impl ::core::marker::Copy for XPS_COLOR_INTERPOLATION {}
impl ::core::clone::Clone for XPS_COLOR_INTERPOLATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_COLOR_INTERPOLATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_INTERPOLATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_COLOR_INTERPOLATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_INTERPOLATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_COLOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
impl ::core::marker::Copy for XPS_COLOR_TYPE {}
impl ::core::clone::Clone for XPS_COLOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_COLOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_COLOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl ::core::marker::Copy for XPS_DASH {}
impl ::core::clone::Clone for XPS_DASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_DASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_DASH").field("length", &self.length).field("gap", &self.gap).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_DASH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_DASH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_DASH>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_DASH {}
impl ::core::default::Default for XPS_DASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_DASH_CAP(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
impl ::core::marker::Copy for XPS_DASH_CAP {}
impl ::core::clone::Clone for XPS_DASH_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_DASH_CAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_DASH_CAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_DASH_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DASH_CAP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
impl ::core::marker::Copy for XPS_DOCUMENT_TYPE {}
impl ::core::clone::Clone for XPS_DOCUMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_DOCUMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_DOCUMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_DOCUMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DOCUMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108159i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ALREADY_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108413i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108407i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108409i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108408i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108923i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108922i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108410i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108671i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_NAMES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109175i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109184i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108416i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_BLEED_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109692i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109685i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109682i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109689i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109686i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109696i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109690i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109684i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109695i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108158i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108160i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109681i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_PAGE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109693i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109694i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108789i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109691i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_XML_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109683i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108924i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108925i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108926i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108791i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109436i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109422i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109431i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109432i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_FONTURI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109433i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_GLYPHS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109438i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109426i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_LOOKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109439i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109440i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109428i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109427i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109424i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109421i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109430i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109429i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109435i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109425i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109434i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109423i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109437i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109182i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109178i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109177i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109179i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109176i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109183i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109180i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109181i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NEGATIVE_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108918i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108670i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108405i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108414i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_OBJECT_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108790i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ODD_BIDILEVEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108921i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108920i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108793i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108794i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108404i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108406i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108412i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108919i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGNATUREID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108792i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGREQUESTID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108795i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_STRING_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108928i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_TOO_MANY_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108927i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109420i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108411i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109688i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109680i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109679i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108415i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108672i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_FILL_RULE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
impl ::core::marker::Copy for XPS_FILL_RULE {}
impl ::core::clone::Clone for XPS_FILL_RULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_FILL_RULE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_FILL_RULE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_FILL_RULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FILL_RULE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_FONT_EMBEDDING(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
impl ::core::marker::Copy for XPS_FONT_EMBEDDING {}
impl ::core::clone::Clone for XPS_FONT_EMBEDDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_FONT_EMBEDDING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_FONT_EMBEDDING {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_FONT_EMBEDDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FONT_EMBEDDING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl ::core::marker::Copy for XPS_GLYPH_INDEX {}
impl ::core::clone::Clone for XPS_GLYPH_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_GLYPH_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_INDEX").field("index", &self.index).field("advanceWidth", &self.advanceWidth).field("horizontalOffset", &self.horizontalOffset).field("verticalOffset", &self.verticalOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_GLYPH_INDEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_GLYPH_INDEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_GLYPH_INDEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_INDEX {}
impl ::core::default::Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl ::core::marker::Copy for XPS_GLYPH_MAPPING {}
impl ::core::clone::Clone for XPS_GLYPH_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_GLYPH_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_MAPPING").field("unicodeStringStart", &self.unicodeStringStart).field("unicodeStringLength", &self.unicodeStringLength).field("glyphIndicesStart", &self.glyphIndicesStart).field("glyphIndicesLength", &self.glyphIndicesLength).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_GLYPH_MAPPING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_GLYPH_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_GLYPH_MAPPING>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_MAPPING {}
impl ::core::default::Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_IMAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
impl ::core::marker::Copy for XPS_IMAGE_TYPE {}
impl ::core::clone::Clone for XPS_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_INTERLEAVING(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
impl ::core::marker::Copy for XPS_INTERLEAVING {}
impl ::core::clone::Clone for XPS_INTERLEAVING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_INTERLEAVING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_INTERLEAVING {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_INTERLEAVING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_INTERLEAVING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_LINE_CAP(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
impl ::core::marker::Copy for XPS_LINE_CAP {}
impl ::core::clone::Clone for XPS_LINE_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_LINE_CAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_LINE_CAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_LINE_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_CAP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_LINE_JOIN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
impl ::core::marker::Copy for XPS_LINE_JOIN {}
impl ::core::clone::Clone for XPS_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_LINE_JOIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_LINE_JOIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_LINE_JOIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_JOIN").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl ::core::marker::Copy for XPS_MATRIX {}
impl ::core::clone::Clone for XPS_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("m31", &self.m31).field("m32", &self.m32).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_MATRIX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_MATRIX>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_MATRIX {}
impl ::core::default::Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
impl ::core::marker::Copy for XPS_OBJECT_TYPE {}
impl ::core::clone::Clone for XPS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for XPS_POINT {}
impl ::core::clone::Clone for XPS_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_POINT").field("x", &self.x).field("y", &self.y).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_POINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_POINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_POINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_POINT {}
impl ::core::default::Default for XPS_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_RECT {}
impl ::core::clone::Clone for XPS_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_RECT").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_RECT {}
impl ::core::default::Default for XPS_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
impl ::core::marker::Copy for XPS_SEGMENT_STROKE_PATTERN {}
impl ::core::clone::Clone for XPS_SEGMENT_STROKE_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SEGMENT_STROKE_PATTERN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SEGMENT_STROKE_PATTERN {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SEGMENT_STROKE_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_STROKE_PATTERN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SEGMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
impl ::core::marker::Copy for XPS_SEGMENT_TYPE {}
impl ::core::clone::Clone for XPS_SEGMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SEGMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SEGMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SEGMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
impl ::core::marker::Copy for XPS_SIGNATURE_STATUS {}
impl ::core::clone::Clone for XPS_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGNATURE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SIGN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
impl ::core::marker::Copy for XPS_SIGN_FLAGS {}
impl ::core::clone::Clone for XPS_SIGN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SIGN_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
impl ::core::marker::Copy for XPS_SIGN_POLICY {}
impl ::core::clone::Clone for XPS_SIGN_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGN_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGN_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SIGN_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_POLICY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_SIZE {}
impl ::core::clone::Clone for XPS_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_SIZE").field("width", &self.width).field("height", &self.height).finish()
    }
}
unsafe impl ::windows::core::Abi for XPS_SIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_SIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_SIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_SIZE {}
impl ::core::default::Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_SPREAD_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
impl ::core::marker::Copy for XPS_SPREAD_METHOD {}
impl ::core::clone::Clone for XPS_SPREAD_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SPREAD_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_SPREAD_METHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_SPREAD_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SPREAD_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_STYLE_SIMULATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
impl ::core::marker::Copy for XPS_STYLE_SIMULATION {}
impl ::core::clone::Clone for XPS_STYLE_SIMULATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_STYLE_SIMULATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_STYLE_SIMULATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_STYLE_SIMULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_STYLE_SIMULATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
impl ::core::marker::Copy for XPS_THUMBNAIL_SIZE {}
impl ::core::clone::Clone for XPS_THUMBNAIL_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_THUMBNAIL_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_THUMBNAIL_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_THUMBNAIL_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_THUMBNAIL_SIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_TILE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
impl ::core::marker::Copy for XPS_TILE_MODE {}
impl ::core::clone::Clone for XPS_TILE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_TILE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XPS_TILE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_TILE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_TILE_MODE").field(&self.0).finish()
    }
}
pub const XpsOMObjectFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);
#[cfg(feature = "implement")]
::core::include!("impl.rs");

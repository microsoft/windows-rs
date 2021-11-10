#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
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
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICE_CAPABILITIES(pub u32);
pub const DC_BINNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(12u32);
pub const DC_BINS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(6u32);
pub const DC_COLLATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(22u32);
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(32u32);
pub const DC_COPIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(18u32);
pub const DC_DRIVER: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(11u32);
pub const DC_DUPLEX: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(7u32);
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(13u32);
pub const DC_EXTRA: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(9u32);
pub const DC_FIELDS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(1u32);
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(14u32);
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(5u32);
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(29u32);
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(34u32);
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(35u32);
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(4u32);
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(17u32);
pub const DC_NUP: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(33u32);
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(16u32);
pub const DC_PAPERS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(2u32);
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(3u32);
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(25u32);
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(28u32);
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(26u32);
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(31u32);
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(27u32);
pub const DC_SIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(8u32);
pub const DC_STAPLE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(30u32);
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(15u32);
pub const DC_VERSION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(10u32);
impl ::core::convert::From<u32> for DEVICE_CAPABILITIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEVICE_CAPABILITIES {
    type Abi = Self;
}
impl ::core::ops::BitOr for DEVICE_CAPABILITIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DEVICE_CAPABILITIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DEVICE_CAPABILITIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DEVICE_CAPABILITIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DEVICE_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PSTR,
    pub lpszOutput: super::super::Foundation::PSTR,
    pub lpszDatatype: super::super::Foundation::PSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DOCINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOCINFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DOCINFOA").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOCINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOCINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOCINFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PWSTR,
    pub lpszOutput: super::super::Foundation::PWSTR,
    pub lpszDatatype: super::super::Foundation::PWSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DOCINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOCINFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DOCINFOW").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOCINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOCINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOCINFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRAWPATRECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DRAWPATRECT").field("ptPosition", &self.ptPosition).field("ptSize", &self.ptSize).field("wStyle", &self.wStyle).field("wPattern", &self.wPattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRAWPATRECT {
    fn eq(&self, other: &Self) -> bool {
        self.ptPosition == other.ptPosition && self.ptSize == other.ptSize && self.wStyle == other.wStyle && self.wPattern == other.wPattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DRAWPATRECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceCapabilitiesA(pdevice: super::super::Foundation::PSTR, pport: super::super::Foundation::PSTR, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32;
        }
        ::core::mem::transmute(DeviceCapabilitiesA(pdevice.into_param().abi(), pport.into_param().abi(), ::core::mem::transmute(fwcapability), ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceCapabilitiesW(pdevice: super::super::Foundation::PWSTR, pport: super::super::Foundation::PWSTR, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32;
        }
        ::core::mem::transmute(DeviceCapabilitiesW(pdevice.into_param().abi(), pport.into_param().abi(), ::core::mem::transmute(fwcapability), ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
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
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
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
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn Escape<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hdc: Param0, iescape: i32, cjin: i32, pvin: Param3, pvout: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Escape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjin: i32, pvin: super::super::Foundation::PSTR, pvout: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(Escape(hdc.into_param().abi(), ::core::mem::transmute(iescape), ::core::mem::transmute(cjin), pvin.into_param().abi(), ::core::mem::transmute(pvout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ExtEscape<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hdc: Param0, iescape: i32, cjinput: i32, lpindata: Param3, cjoutput: i32, lpoutdata: super::super::Foundation::PSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtEscape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjinput: i32, lpindata: super::super::Foundation::PSTR, cjoutput: i32, lpoutdata: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(ExtEscape(hdc.into_param().abi(), ::core::mem::transmute(iescape), ::core::mem::transmute(cjinput), lpindata.into_param().abi(), ::core::mem::transmute(cjoutput), ::core::mem::transmute(lpoutdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HPTPROVIDER(pub isize);
impl ::core::default::Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPTPROVIDER {}
unsafe impl ::windows::runtime::Abi for HPTPROVIDER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsDocumentPackageTarget(pub ::windows::runtime::IUnknown);
impl IXpsDocumentPackageTarget {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetXpsOMPackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, discardcontrolpartname: Param1) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::runtime::Result<IXpsOMObjectFactory> {
        let mut result__: <IXpsOMObjectFactory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMObjectFactory>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetXpsType(&self) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsDocumentPackageTarget {
    type Vtable = IXpsDocumentPackageTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3b0b6d38_53ad_41da_b212_d37637a6714e);
}
impl ::core::convert::From<IXpsDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: IXpsDocumentPackageTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsDocumentPackageTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequencepartname: ::windows::runtime::RawPtr, discardcontrolpartname: ::windows::runtime::RawPtr, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpsfactory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsDocumentPackageTarget3D(pub ::windows::runtime::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetXpsOMPackageWriter3D<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(
        &self,
        documentsequencepartname: Param0,
        discardcontrolpartname: Param1,
        modelpartname: Param2,
        modeldata: Param3,
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter3D> {
        let mut result__: <IXpsOMPackageWriter3D as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), modelpartname.into_param().abi(), modeldata.into_param().abi(), &mut result__).from_abi::<IXpsOMPackageWriter3D>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::runtime::Result<IXpsOMObjectFactory> {
        let mut result__: <IXpsOMObjectFactory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMObjectFactory>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsDocumentPackageTarget3D {
    type Vtable = IXpsDocumentPackageTarget3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x60ba71b8_3101_4984_9199_f4ea775ff01d);
}
impl ::core::convert::From<IXpsDocumentPackageTarget3D> for ::windows::runtime::IUnknown {
    fn from(value: IXpsDocumentPackageTarget3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsDocumentPackageTarget3D> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsDocumentPackageTarget3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsDocumentPackageTarget3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsDocumentPackageTarget3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequencepartname: ::windows::runtime::RawPtr, discardcontrolpartname: ::windows::runtime::RawPtr, modelpartname: ::windows::runtime::RawPtr, modeldata: ::windows::runtime::RawPtr, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpsfactory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMBrush {
    type Vtable = IXpsOMBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
}
impl ::core::convert::From<IXpsOMBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMCanvas(pub ::windows::runtime::IUnknown);
impl IXpsOMCanvas {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::runtime::Result<super::super::System::Com::IUri> {
        let mut result__: <super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::runtime::Result<IXpsOMVisualCollection> {
        let mut result__: <IXpsOMVisualCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetUseAliasedEdgeMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, usealiasededgemode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), usealiasededgemode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, shortdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, longdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMCanvas> {
        let mut result__: <IXpsOMCanvas as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCanvas>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMCanvas {
    type Vtable = IXpsOMCanvas_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
}
impl ::core::convert::From<IXpsOMCanvas> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMCanvas) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMCanvas> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMCanvas) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for &IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMCanvas {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvas_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visuals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, longdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, canvas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMColorProfileResource(pub ::windows::runtime::IUnknown);
impl IXpsOMColorProfileResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMColorProfileResource {
    type Vtable = IXpsOMColorProfileResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
}
impl ::core::convert::From<IXpsOMColorProfileResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMColorProfileResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMColorProfileResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMColorProfileResourceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMColorProfileResource> {
        let mut result__: <IXpsOMColorProfileResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, object: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::runtime::Result<IXpsOMColorProfileResource> {
        let mut result__: <IXpsOMColorProfileResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), &mut result__).from_abi::<IXpsOMColorProfileResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMColorProfileResourceCollection {
    type Vtable = IXpsOMColorProfileResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12759630_5fba_4283_8f7d_cca849809edb);
}
impl ::core::convert::From<IXpsOMColorProfileResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMColorProfileResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMColorProfileResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMColorProfileResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMColorProfileResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMColorProfileResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMCoreProperties(pub ::windows::runtime::IUnknown);
impl IXpsOMCoreProperties {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetCategory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, category: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), category.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetContentStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetContentStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, contentstatus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), contentstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetContentType(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetContentType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, contenttype: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), contenttype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetCreated(&self) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(created)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetCreator(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetCreator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, creator: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), creator.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIdentifier<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, identifier: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), identifier.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetKeywords(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetKeywords<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, keywords: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), keywords.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLastModifiedBy(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLastModifiedBy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lastmodifiedby: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), lastmodifiedby.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLastPrinted(&self) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(lastprinted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetModified(&self) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(modified)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetRevision<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, revision: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), revision.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSubject(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, subject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), subject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTitle(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, title: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), title.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, version: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMCoreProperties> {
        let mut result__: <IXpsOMCoreProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCoreProperties>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMCoreProperties {
    type Vtable = IXpsOMCoreProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
}
impl ::core::convert::From<IXpsOMCoreProperties> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMCoreProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMCoreProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMCoreProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCoreProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentstatus: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, creator: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, creator: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keywords: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, revision: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, revision: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subject: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subject: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coreproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDashCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMDashCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<XPS_DASH> {
        let mut result__: <XPS_DASH as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<XPS_DASH>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dash)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDashCollection {
    type Vtable = IXpsOMDashCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
}
impl ::core::convert::From<IXpsOMDashCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDashCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDashCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDashCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDashCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDashCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, dash: *mut XPS_DASH) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, dash: *const XPS_DASH) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, dash: *const XPS_DASH) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dash: *const XPS_DASH) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDictionary(pub ::windows::runtime::IUnknown);
impl IXpsOMDictionary {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAt(&self, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(key), ::core::mem::transmute(entry)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetByKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, beforeentry: Param1) -> ::windows::runtime::Result<IXpsOMShareable> {
        let mut result__: <IXpsOMShareable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), key.into_param().abi(), beforeentry.into_param().abi(), &mut result__).from_abi::<IXpsOMShareable>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetIndex<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMShareable>>(&self, entry: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), entry.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, entry: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDictionary {
    type Vtable = IXpsOMDictionary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
}
impl ::core::convert::From<IXpsOMDictionary> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDictionary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDictionary> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDictionary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDictionary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDictionary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDictionary_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR, beforeentry: ::windows::runtime::RawPtr, entry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, entry: ::windows::runtime::RawPtr, index: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR, entry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDocument(pub ::windows::runtime::IUnknown);
impl IXpsOMDocument {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMDocumentSequence> {
        let mut result__: <IXpsOMDocumentSequence as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPageReferences(&self) -> ::windows::runtime::Result<IXpsOMPageReferenceCollection> {
        let mut result__: <IXpsOMPageReferenceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPageReferenceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::runtime::Result<IXpsOMPrintTicketResource> {
        let mut result__: <IXpsOMPrintTicketResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentStructureResource(&self) -> ::windows::runtime::Result<IXpsOMDocumentStructureResource> {
        let mut result__: <IXpsOMDocumentStructureResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDocumentStructureResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDocumentStructureResource>>(&self, documentstructureresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), documentstructureresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignatureBlockResources(&self) -> ::windows::runtime::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__: <IXpsOMSignatureBlockResourceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMSignatureBlockResourceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDocument {
    type Vtable = IXpsOMDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
}
impl ::core::convert::From<IXpsOMDocument> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDocument> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequence: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagereferences: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentstructureresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentstructureresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureblockresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, document: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDocumentCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMDocumentCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDocument>>(&self, document: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), document.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDocumentCollection {
    type Vtable = IXpsOMDocumentCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd1c87f0d_e947_4754_8a25_971478f7e83e);
}
impl ::core::convert::From<IXpsOMDocumentCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDocumentCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDocumentCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDocumentCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDocumentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDocumentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, document: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, document: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, document: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, document: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDocumentSequence(pub ::windows::runtime::IUnknown);
impl IXpsOMDocumentSequence {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocuments(&self) -> ::windows::runtime::Result<IXpsOMDocumentCollection> {
        let mut result__: <IXpsOMDocumentCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocumentCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::runtime::Result<IXpsOMPrintTicketResource> {
        let mut result__: <IXpsOMPrintTicketResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDocumentSequence {
    type Vtable = IXpsOMDocumentSequence_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
}
impl ::core::convert::From<IXpsOMDocumentSequence> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDocumentSequence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDocumentSequence> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDocumentSequence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequence_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMDocumentStructureResource(pub ::windows::runtime::IUnknown);
impl IXpsOMDocumentStructureResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMDocumentStructureResource {
    type Vtable = IXpsOMDocumentStructureResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85febc8a_6b63_48a9_af07_7064e4ecff30);
}
impl ::core::convert::From<IXpsOMDocumentStructureResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMDocumentStructureResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMDocumentStructureResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMDocumentStructureResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentStructureResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMFontResource(pub ::windows::runtime::IUnknown);
impl IXpsOMFontResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, embeddingoption: XPS_FONT_EMBEDDING, partname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(embeddingoption), partname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetEmbeddingOption(&self) -> ::windows::runtime::Result<XPS_FONT_EMBEDDING> {
        let mut result__: <XPS_FONT_EMBEDDING as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_FONT_EMBEDDING>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMFontResource {
    type Vtable = IXpsOMFontResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
}
impl ::core::convert::From<IXpsOMFontResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMFontResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMFontResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMFontResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMFontResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, readerstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMFontResourceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMFontResourceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMFontResource> {
        let mut result__: <IXpsOMFontResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::runtime::Result<IXpsOMFontResource> {
        let mut result__: <IXpsOMFontResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), &mut result__).from_abi::<IXpsOMFontResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMFontResourceCollection {
    type Vtable = IXpsOMFontResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
}
impl ::core::convert::From<IXpsOMFontResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMFontResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMFontResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMFontResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMFontResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMFontResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGeometry(pub ::windows::runtime::IUnknown);
impl IXpsOMGeometry {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFigures(&self) -> ::windows::runtime::Result<IXpsOMGeometryFigureCollection> {
        let mut result__: <IXpsOMGeometryFigureCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometryFigureCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFillRule(&self) -> ::windows::runtime::Result<XPS_FILL_RULE> {
        let mut result__: <XPS_FILL_RULE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_FILL_RULE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fillrule)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGeometry {
    type Vtable = IXpsOMGeometry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
}
impl ::core::convert::From<IXpsOMGeometry> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGeometry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGeometry> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGeometry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGeometry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGeometry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMGeometry {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMGeometry {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, figures: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fillrule: *mut XPS_FILL_RULE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fillrule: XPS_FILL_RULE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGeometryFigure(pub ::windows::runtime::IUnknown);
impl IXpsOMGeometryFigure {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(datacount), ::core::mem::transmute(segmentdata)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmenttypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentstrokes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentdatacount), ::core::mem::transmute(segmenttypes), ::core::mem::transmute(segmentdata), ::core::mem::transmute(segmentstrokes)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStartPoint(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsClosed(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isclosed: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), isclosed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsFilled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsFilled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isfilled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), isfilled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSegmentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSegmentDataCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSegmentStrokePattern(&self) -> ::windows::runtime::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__: <XPS_SEGMENT_STROKE_PATTERN as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SEGMENT_STROKE_PATTERN>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMGeometryFigure> {
        let mut result__: <IXpsOMGeometryFigure as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometryFigure>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGeometryFigure {
    type Vtable = IXpsOMGeometryFigure_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd410dc83_908c_443e_8947_b1795d3c165a);
}
impl ::core::convert::From<IXpsOMGeometryFigure> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGeometryFigure) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGeometryFigure> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGeometryFigure) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGeometryFigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGeometryFigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigure_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isclosed: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isfilled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentdatacount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometryfigure: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGeometryFigureCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMGeometryFigureCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMGeometryFigure> {
        let mut result__: <IXpsOMGeometryFigure as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometryFigure>>(&self, geometryfigure: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), geometryfigure.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGeometryFigureCollection {
    type Vtable = IXpsOMGeometryFigureCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
}
impl ::core::convert::From<IXpsOMGeometryFigureCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGeometryFigureCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGeometryFigureCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGeometryFigureCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGeometryFigureCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGeometryFigureCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, geometryfigure: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, geometryfigure: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, geometryfigure: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometryfigure: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGlyphs(pub ::windows::runtime::IUnknown);
impl IXpsOMGlyphs {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::runtime::Result<super::super::System::Com::IUri> {
        let mut result__: <super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(prohibitedcaretstopcount), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetBidiLevel(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsSideways(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStyleSimulations(&self) -> ::windows::runtime::Result<XPS_STYLE_SIMULATION> {
        let mut result__: <XPS_STYLE_SIMULATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_STYLE_SIMULATION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(stylesimulations)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOrigin(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFontRenderingEmSize(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontrenderingemsize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFontResource(&self) -> ::windows::runtime::Result<IXpsOMFontResource> {
        let mut result__: <IXpsOMFontResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFontResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), fontresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFontFaceIndex(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontfaceindex)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFillBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, fillbrush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), fillbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphsEditor(&self) -> ::windows::runtime::Result<IXpsOMGlyphsEditor> {
        let mut result__: <IXpsOMGlyphsEditor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGlyphsEditor>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMGlyphs> {
        let mut result__: <IXpsOMGlyphs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGlyphs>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGlyphs {
    type Vtable = IXpsOMGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
}
impl ::core::convert::From<IXpsOMGlyphs> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGlyphs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGlyphs> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGlyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for &IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphmappingcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prohibitedcaretstopcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bidilevel: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, issideways: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, origin: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, origin: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontrenderingemsize: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontrenderingemsize: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontfaceindex: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontfaceindex: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fillbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fillbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fillbrush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, editor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGlyphsEditor(pub ::windows::runtime::IUnknown);
impl IXpsOMGlyphsEditor {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn ApplyEdits(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetUnicodeString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, unicodestring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), unicodestring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetBidiLevel(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bidilevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsSideways(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsSideways<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, issideways: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), issideways.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetDeviceFontName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, devicefontname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), devicefontname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGlyphsEditor {
    type Vtable = IXpsOMGlyphsEditor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
}
impl ::core::convert::From<IXpsOMGlyphsEditor> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGlyphsEditor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGlyphsEditor> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGlyphsEditor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGlyphsEditor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGlyphsEditor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsEditor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicodestring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphmappingcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prohibitedcaretstopcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, prohibitedcaretstops: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bidilevel: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bidilevel: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, issideways: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, issideways: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicefontname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGradientBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMGradientBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::runtime::Result<IXpsOMGradientStopCollection> {
        let mut result__: <IXpsOMGradientStopCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::runtime::Result<XPS_SPREAD_METHOD> {
        let mut result__: <XPS_SPREAD_METHOD as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::runtime::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: <XPS_COLOR_INTERPOLATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGradientBrush {
    type Vtable = IXpsOMGradientBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xedb59622_61a2_42c3_bace_acf2286c06bf);
}
impl ::core::convert::From<IXpsOMGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGradientBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGradientBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradientstops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGradientStop(pub ::windows::runtime::IUnknown);
impl IXpsOMGradientStop {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMGradientBrush> {
        let mut result__: <IXpsOMGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGradientBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOffset(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOffset(&self, offset: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColor<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMGradientStop> {
        let mut result__: <IXpsOMGradientStop as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGradientStop>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGradientStop {
    type Vtable = IXpsOMGradientStop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
}
impl ::core::convert::From<IXpsOMGradientStop> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGradientStop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGradientStop> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGradientStop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGradientStop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGradientStop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *mut XPS_COLOR, colorprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradientstop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMGradientStopCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMGradientStopCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMGradientStop> {
        let mut result__: <IXpsOMGradientStop as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, stop: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), stop.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMGradientStopCollection {
    type Vtable = IXpsOMGradientStopCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
}
impl ::core::convert::From<IXpsOMGradientStopCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMGradientStopCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMGradientStopCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMGradientStopCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMGradientStopCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMGradientStopCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, stop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, stop: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, stop: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stop: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMImageBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMImageBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::runtime::Result<XPS_TILE_MODE> {
        let mut result__: <XPS_TILE_MODE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetImageResource(&self) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetImageResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColorProfileResource(&self) -> ::windows::runtime::Result<IXpsOMColorProfileResource> {
        let mut result__: <IXpsOMColorProfileResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColorProfileResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, colorprofileresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), colorprofileresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMImageBrush> {
        let mut result__: <IXpsOMImageBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageBrush>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMImageBrush {
    type Vtable = IXpsOMImageBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3df0b466_d382_49ef_8550_dd94c80242e4);
}
impl ::core::convert::From<IXpsOMImageBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMImageBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMImageBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMImageBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMTileBrush> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMTileBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMTileBrush> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMTileBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: *mut XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorprofileresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorprofileresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagebrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMImageResource(pub ::windows::runtime::IUnknown);
impl IXpsOMImageResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, imagetype: XPS_IMAGE_TYPE, partname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(imagetype), partname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetImageType(&self) -> ::windows::runtime::Result<XPS_IMAGE_TYPE> {
        let mut result__: <XPS_IMAGE_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_IMAGE_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMImageResource {
    type Vtable = IXpsOMImageResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3db8417d_ae50_485e_9a44_d7758f78a23f);
}
impl ::core::convert::From<IXpsOMImageResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMImageResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMImageResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMImageResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMImageResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, readerstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMImageResourceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMImageResourceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, object: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMImageResourceCollection {
    type Vtable = IXpsOMImageResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
}
impl ::core::convert::From<IXpsOMImageResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMImageResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMImageResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMImageResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMImageResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMImageResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMLinearGradientBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMLinearGradientBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::runtime::Result<IXpsOMGradientStopCollection> {
        let mut result__: <IXpsOMGradientStopCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::runtime::Result<XPS_SPREAD_METHOD> {
        let mut result__: <XPS_SPREAD_METHOD as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::runtime::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: <XPS_COLOR_INTERPOLATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStartPoint(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetEndPoint(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(endpoint)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMLinearGradientBrush> {
        let mut result__: <IXpsOMLinearGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMLinearGradientBrush {
    type Vtable = IXpsOMLinearGradientBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x005e279f_c30d_40ff_93ec_1950d3c528db);
}
impl ::core::convert::From<IXpsOMLinearGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMLinearGradientBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMLinearGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMLinearGradientBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMGradientBrush> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMGradientBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMGradientBrush> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMGradientBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradientstops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpoint: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpoint: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lineargradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMMatrixTransform(pub ::windows::runtime::IUnknown);
impl IXpsOMMatrixTransform {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetMatrix(&self) -> ::windows::runtime::Result<XPS_MATRIX> {
        let mut result__: <XPS_MATRIX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_MATRIX>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMMatrixTransform {
    type Vtable = IXpsOMMatrixTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
}
impl ::core::convert::From<IXpsOMMatrixTransform> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMMatrixTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMMatrixTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMMatrixTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *mut XPS_MATRIX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const XPS_MATRIX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMNameCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMNameCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMNameCollection {
    type Vtable = IXpsOMNameCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4bddf8ec_c915_421b_a166_d173d25653d2);
}
impl ::core::convert::From<IXpsOMNameCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMNameCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMNameCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMNameCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMNameCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMNameCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMObjectFactory(pub ::windows::runtime::IUnknown);
impl IXpsOMObjectFactory {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePackage(&self) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: <IXpsOMStoryFragmentsResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMDocumentStructureResource> {
        let mut result__: <IXpsOMDocumentStructureResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMSignatureBlockResource> {
        let mut result__: <IXpsOMSignatureBlockResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePartResources(&self) -> ::windows::runtime::Result<IXpsOMPartResources> {
        let mut result__: <IXpsOMPartResources as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMDocumentSequence> {
        let mut result__: <IXpsOMDocumentSequence as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateCanvas(&self) -> ::windows::runtime::Result<IXpsOMCanvas> {
        let mut result__: <IXpsOMCanvas as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCanvas>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::runtime::Result<IXpsOMGlyphs> {
        let mut result__: <IXpsOMGlyphs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), &mut result__).from_abi::<IXpsOMGlyphs>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePath(&self) -> ::windows::runtime::Result<IXpsOMPath> {
        let mut result__: <IXpsOMPath as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPath>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::runtime::Result<IXpsOMGeometryFigure> {
        let mut result__: <IXpsOMGeometryFigure as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), &mut result__).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::runtime::Result<IXpsOMSolidColorBrush> {
        let mut result__: <IXpsOMSolidColorBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), &mut result__).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMColorProfileResource> {
        let mut result__: <IXpsOMColorProfileResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::runtime::Result<IXpsOMImageBrush> {
        let mut result__: <IXpsOMImageBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), &mut result__).from_abi::<IXpsOMImageBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::runtime::Result<IXpsOMVisualBrush> {
        let mut result__: <IXpsOMVisualBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), &mut result__).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMPrintTicketResource> {
        let mut result__: <IXpsOMPrintTicketResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::runtime::Result<IXpsOMFontResource> {
        let mut result__: <IXpsOMFontResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), &mut result__).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::runtime::Result<IXpsOMGradientStop> {
        let mut result__: <IXpsOMGradientStop as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), &mut result__).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::runtime::Result<IXpsOMLinearGradientBrush> {
        let mut result__: <IXpsOMLinearGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), &mut result__).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::runtime::Result<IXpsOMRadialGradientBrush> {
        let mut result__: <IXpsOMRadialGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), &mut result__).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMCoreProperties> {
        let mut result__: <IXpsOMCoreProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::runtime::Result<IXpsOMPartUriCollection> {
        let mut result__: <IXpsOMPartUriCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePackageWriterOnFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param7: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param8: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param9: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
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
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
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
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageWriterOnStream<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param4: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param5: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param7: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
        &self,
        outputstream: Param0,
        optimizemarkupsize: Param1,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param3,
        coreproperties: Param4,
        packagethumbnail: Param5,
        documentsequenceprintticket: Param6,
        discardcontrolpartname: Param7,
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::core::mem::transmute_copy(self),
            outputstream.into_param().abi(),
            optimizemarkupsize.into_param().abi(),
            ::core::mem::transmute(interleaving),
            documentsequencepartname.into_param().abi(),
            coreproperties.into_param().abi(),
            packagethumbnail.into_param().abi(),
            documentsequenceprintticket.into_param().abi(),
            discardcontrolpartname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uri: Param0) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), uri.into_param().abi(), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMObjectFactory {
    type Vtable = IXpsOMObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
}
impl ::core::convert::From<IXpsOMObjectFactory> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMObjectFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMObjectFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMObjectFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMObjectFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMObjectFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, storyfragmentsresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, documentstructureresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, signatureblockresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionarymarkupstream: ::windows::runtime::RawPtr, dictionaryparturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, dictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, documentsequence: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, document: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagemarkupstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, canvas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontresource: ::windows::runtime::RawPtr, glyphs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT, figure: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const XPS_MATRIX, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr, solidcolorbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, colorprofileresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, printticketresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::runtime::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr, offset: f32, gradientstop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradstop1: ::windows::runtime::RawPtr, gradstop2: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradstop1: ::windows::runtime::RawPtr, gradstop2: ::windows::runtime::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, coreproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturicollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: super::super::Foundation::PWSTR,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        outputstream: ::windows::runtime::RawPtr,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMObjectFactory1(pub ::windows::runtime::IUnknown);
impl IXpsOMObjectFactory1 {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePackage(&self) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage> {
        let mut result__: <IXpsOMPackage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: <IXpsOMStoryFragmentsResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMDocumentStructureResource> {
        let mut result__: <IXpsOMDocumentStructureResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMSignatureBlockResource> {
        let mut result__: <IXpsOMSignatureBlockResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePartResources(&self) -> ::windows::runtime::Result<IXpsOMPartResources> {
        let mut result__: <IXpsOMPartResources as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMDocumentSequence> {
        let mut result__: <IXpsOMDocumentSequence as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateCanvas(&self) -> ::windows::runtime::Result<IXpsOMCanvas> {
        let mut result__: <IXpsOMCanvas as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCanvas>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::runtime::Result<IXpsOMGlyphs> {
        let mut result__: <IXpsOMGlyphs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), &mut result__).from_abi::<IXpsOMGlyphs>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePath(&self) -> ::windows::runtime::Result<IXpsOMPath> {
        let mut result__: <IXpsOMPath as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPath>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::runtime::Result<IXpsOMGeometryFigure> {
        let mut result__: <IXpsOMGeometryFigure as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), &mut result__).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::runtime::Result<IXpsOMSolidColorBrush> {
        let mut result__: <IXpsOMSolidColorBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), &mut result__).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMColorProfileResource> {
        let mut result__: <IXpsOMColorProfileResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::runtime::Result<IXpsOMImageBrush> {
        let mut result__: <IXpsOMImageBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), &mut result__).from_abi::<IXpsOMImageBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::runtime::Result<IXpsOMVisualBrush> {
        let mut result__: <IXpsOMVisualBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), &mut result__).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::runtime::Result<IXpsOMPrintTicketResource> {
        let mut result__: <IXpsOMPrintTicketResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::runtime::Result<IXpsOMFontResource> {
        let mut result__: <IXpsOMFontResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), &mut result__).from_abi::<IXpsOMFontResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::runtime::Result<IXpsOMGradientStop> {
        let mut result__: <IXpsOMGradientStop as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), &mut result__).from_abi::<IXpsOMGradientStop>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::runtime::Result<IXpsOMLinearGradientBrush> {
        let mut result__: <IXpsOMLinearGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), &mut result__).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::runtime::Result<IXpsOMRadialGradientBrush> {
        let mut result__: <IXpsOMRadialGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), &mut result__).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<IXpsOMCoreProperties> {
        let mut result__: <IXpsOMCoreProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::runtime::Result<IXpsOMPartUriCollection> {
        let mut result__: <IXpsOMPartUriCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePackageWriterOnFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param7: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param8: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param9: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
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
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
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
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageWriterOnStream<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param4: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param5: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param7: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
        &self,
        outputstream: Param0,
        optimizemarkupsize: Param1,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: Param3,
        coreproperties: Param4,
        packagethumbnail: Param5,
        documentsequenceprintticket: Param6,
        discardcontrolpartname: Param7,
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::core::mem::transmute_copy(self),
            outputstream.into_param().abi(),
            optimizemarkupsize.into_param().abi(),
            ::core::mem::transmute(interleaving),
            documentsequencepartname.into_param().abi(),
            coreproperties.into_param().abi(),
            packagethumbnail.into_param().abi(),
            documentsequenceprintticket.into_param().abi(),
            discardcontrolpartname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uri: Param0) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), uri.into_param().abi(), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetDocumentTypeFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetDocumentTypeFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, xpsdocumentstream: Param0) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), xpsdocumentstream.into_param().abi(), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn ConvertHDPhotoToJpegXR<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn ConvertJpegXRToHDPhoto<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePackageWriterOnFile1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param7: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param8: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param9: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
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
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
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
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageWriterOnStream1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
        Param4: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>,
        Param5: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>,
        Param6: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>,
        Param7: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>,
    >(
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
    ) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::core::mem::transmute_copy(self),
            outputstream.into_param().abi(),
            optimizemarkupsize.into_param().abi(),
            ::core::mem::transmute(interleaving),
            documentsequencepartname.into_param().abi(),
            coreproperties.into_param().abi(),
            packagethumbnail.into_param().abi(),
            documentsequenceprintticket.into_param().abi(),
            discardcontrolpartname.into_param().abi(),
            ::core::mem::transmute(documenttype),
            &mut result__,
        )
        .from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreatePackage1(&self) -> ::windows::runtime::Result<IXpsOMPackage1> {
        let mut result__: <IXpsOMPackage1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageFromStream1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage1> {
        let mut result__: <IXpsOMPackage1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn CreatePackageFromFile1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::runtime::Result<IXpsOMPackage1> {
        let mut result__: <IXpsOMPackage1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePage1<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::runtime::Result<IXpsOMPage1> {
        let mut result__: <IXpsOMPage1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi::<IXpsOMPage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreatePageFromStream1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::runtime::Result<IXpsOMPage1> {
        let mut result__: <IXpsOMPage1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi::<IXpsOMPage1>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, parturi: Param1, resources: Param2) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMObjectFactory1 {
    type Vtable = IXpsOMObjectFactory1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
}
impl ::core::convert::From<IXpsOMObjectFactory1> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMObjectFactory1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMObjectFactory1> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMObjectFactory1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMObjectFactory> for IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMObjectFactory> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMObjectFactory> for &IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMObjectFactory> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, storyfragmentsresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, documentstructureresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, signatureblockresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionarymarkupstream: ::windows::runtime::RawPtr, dictionaryparturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, dictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, documentsequence: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, document: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagemarkupstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, canvas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontresource: ::windows::runtime::RawPtr, glyphs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT, figure: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const XPS_MATRIX, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr, solidcolorbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, colorprofileresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, printticketresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acquiredstream: ::windows::runtime::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::runtime::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr, offset: f32, gradientstop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradstop1: ::windows::runtime::RawPtr, gradstop2: ::windows::runtime::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradstop1: ::windows::runtime::RawPtr, gradstop2: ::windows::runtime::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, coreproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturicollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: super::super::Foundation::PWSTR,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        outputstream: ::windows::runtime::RawPtr,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpsdocumentstream: ::windows::runtime::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: super::super::Foundation::PWSTR,
        securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        flagsandattributes: u32,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        documenttype: XPS_DOCUMENT_TYPE,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        outputstream: ::windows::runtime::RawPtr,
        optimizemarkupsize: super::super::Foundation::BOOL,
        interleaving: XPS_INTERLEAVING,
        documentsequencepartname: ::windows::runtime::RawPtr,
        coreproperties: ::windows::runtime::RawPtr,
        packagethumbnail: ::windows::runtime::RawPtr,
        documentsequenceprintticket: ::windows::runtime::RawPtr,
        discardcontrolpartname: ::windows::runtime::RawPtr,
        documenttype: XPS_DOCUMENT_TYPE,
        packagewriter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagemarkupstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionarymarkupstream: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, resources: ::windows::runtime::RawPtr, dictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPackage(pub ::windows::runtime::IUnknown);
impl IXpsOMPackage {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::runtime::Result<IXpsOMDocumentSequence> {
        let mut result__: <IXpsOMDocumentSequence as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCoreProperties(&self) -> ::windows::runtime::Result<IXpsOMCoreProperties> {
        let mut result__: <IXpsOMCoreProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPackage {
    type Vtable = IXpsOMPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18c3df65_81e1_4674_91dc_fc452f5a416f);
}
impl ::core::convert::From<IXpsOMPackage> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPackage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPackage> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequence: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequence: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coreproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coreproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discardcontrolparturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discardcontrolparturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPackage1(pub ::windows::runtime::IUnknown);
impl IXpsOMPackage1 {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::runtime::Result<IXpsOMDocumentSequence> {
        let mut result__: <IXpsOMDocumentSequence as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCoreProperties(&self) -> ::windows::runtime::Result<IXpsOMCoreProperties> {
        let mut result__: <IXpsOMCoreProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMCoreProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn WriteToFile1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn WriteToStream1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, outputstream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPackage1 {
    type Vtable = IXpsOMPackage1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
}
impl ::core::convert::From<IXpsOMPackage1> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPackage1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPackage1> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPackage1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPackage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPackage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPackage> for IXpsOMPackage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPackage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPackage> for &IXpsOMPackage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPackage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequence: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequence: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coreproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coreproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discardcontrolparturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discardcontrolparturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPackageTarget(pub ::windows::runtime::IUnknown);
impl IXpsOMPackageTarget {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateXpsOMPackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, documentsequenceprintticket: Param1, discardcontrolpartname: Param2) -> ::windows::runtime::Result<IXpsOMPackageWriter> {
        let mut result__: <IXpsOMPackageWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi::<IXpsOMPackageWriter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPackageTarget {
    type Vtable = IXpsOMPackageTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
}
impl ::core::convert::From<IXpsOMPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPackageTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPackageTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequencepartname: ::windows::runtime::RawPtr, documentsequenceprintticket: ::windows::runtime::RawPtr, discardcontrolpartname: ::windows::runtime::RawPtr, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPackageWriter(pub ::windows::runtime::IUnknown);
impl IXpsOMPackageWriter {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::runtime::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::runtime::IntoParam<'a, IXpsOMPartUriCollection>>(
        &self,
        documentpartname: Param0,
        documentprintticket: Param1,
        documentstructure: Param2,
        signatureblockresources: Param3,
        restrictedfonts: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPage>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::runtime::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(
        &self,
        page: Param0,
        advisorypagedimensions: *const XPS_SIZE,
        discardableresourceparts: Param2,
        storyfragments: Param3,
        pageprintticket: Param4,
        pagethumbnail: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn IsClosed(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPackageWriter {
    type Vtable = IXpsOMPackageWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
}
impl ::core::convert::From<IXpsOMPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPackageWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPackageWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentpartname: ::windows::runtime::RawPtr, documentprintticket: ::windows::runtime::RawPtr, documentstructure: ::windows::runtime::RawPtr, signatureblockresources: ::windows::runtime::RawPtr, restrictedfonts: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: ::windows::runtime::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::runtime::RawPtr, storyfragments: ::windows::runtime::RawPtr, pageprintticket: ::windows::runtime::RawPtr, pagethumbnail: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPackageWriter3D(pub ::windows::runtime::IUnknown);
impl IXpsOMPackageWriter3D {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::runtime::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::runtime::IntoParam<'a, IXpsOMPartUriCollection>>(
        &self,
        documentpartname: Param0,
        documentprintticket: Param1,
        documentstructure: Param2,
        signatureblockresources: Param3,
        restrictedfonts: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPage>, Param2: ::windows::runtime::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::runtime::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(
        &self,
        page: Param0,
        advisorypagedimensions: *const XPS_SIZE,
        discardableresourceparts: Param2,
        storyfragments: Param3,
        pageprintticket: Param4,
        pagethumbnail: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn AddResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn IsClosed(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn AddModelTexture<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, texturepartname: Param0, texturedata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), texturepartname.into_param().abi(), texturedata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetModelPrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, printticketpartname: Param0, printticketdata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), printticketpartname.into_param().abi(), printticketdata.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPackageWriter3D {
    type Vtable = IXpsOMPackageWriter3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
}
impl ::core::convert::From<IXpsOMPackageWriter3D> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPackageWriter3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPackageWriter3D> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPackageWriter3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPackageWriter> for IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPackageWriter> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPackageWriter> for &IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPackageWriter> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter3D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentpartname: ::windows::runtime::RawPtr, documentprintticket: ::windows::runtime::RawPtr, documentstructure: ::windows::runtime::RawPtr, signatureblockresources: ::windows::runtime::RawPtr, restrictedfonts: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: ::windows::runtime::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::runtime::RawPtr, storyfragments: ::windows::runtime::RawPtr, pageprintticket: ::windows::runtime::RawPtr, pagethumbnail: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, texturepartname: ::windows::runtime::RawPtr, texturedata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketpartname: ::windows::runtime::RawPtr, printticketdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPage(pub ::windows::runtime::IUnknown);
impl IXpsOMPage {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::runtime::Result<IXpsOMVisualCollection> {
        let mut result__: <IXpsOMVisualCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPageDimensions(&self) -> ::windows::runtime::Result<XPS_SIZE> {
        let mut result__: <XPS_SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetContentBox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetBleedBox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPage {
    type Vtable = IXpsOMPage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd3e18888_f120_4fee_8c68_35296eae91d4);
}
impl ::core::convert::From<IXpsOMPage> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPage> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visuals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *mut XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bleedbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bleedbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPage1(pub ::windows::runtime::IUnknown);
impl IXpsOMPage1 {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetVisuals(&self) -> ::windows::runtime::Result<IXpsOMVisualCollection> {
        let mut result__: <IXpsOMVisualCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPageDimensions(&self) -> ::windows::runtime::Result<XPS_SIZE> {
        let mut result__: <XPS_SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetContentBox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetBleedBox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Write1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPage1 {
    type Vtable = IXpsOMPage1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x305b60ef_6892_4dda_9cbb_3aa65974508a);
}
impl ::core::convert::From<IXpsOMPage1> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPage1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPage1> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPage1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPage> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPage> for &IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMPage1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visuals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *mut XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bleedbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bleedbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotedictionaryresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPageReference(pub ::windows::runtime::IUnknown);
impl IXpsOMPageReference {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPage(&self) -> ::windows::runtime::Result<IXpsOMPage> {
        let mut result__: <IXpsOMPage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPage>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPage<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPage>>(&self, page: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), page.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn DiscardPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn IsPageLoaded(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> ::windows::runtime::Result<XPS_SIZE> {
        let mut result__: <XPS_SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStoryFragmentsResource(&self) -> ::windows::runtime::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: <IXpsOMStoryFragmentsResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStoryFragmentsResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMStoryFragmentsResource>>(&self, storyfragmentsresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), storyfragmentsresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::runtime::Result<IXpsOMPrintTicketResource> {
        let mut result__: <IXpsOMPrintTicketResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CollectLinkTargets(&self) -> ::windows::runtime::Result<IXpsOMNameCollection> {
        let mut result__: <IXpsOMNameCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMNameCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CollectPartResources(&self) -> ::windows::runtime::Result<IXpsOMPartResources> {
        let mut result__: <IXpsOMPartResources as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn HasRestrictedFonts(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPageReference {
    type Vtable = IXpsOMPageReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed360180_6f92_4998_890d_2f208531a0a0);
}
impl ::core::convert::From<IXpsOMPageReference> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPageReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPageReference> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPageReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPageReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPageReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, document: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *mut XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagedimensions: *const XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyfragmentsresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyfragmentsresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticketresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linktargets: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPageReferenceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMPageReferenceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPageReference>>(&self, pagereference: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pagereference.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPageReferenceCollection {
    type Vtable = IXpsOMPageReferenceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca16ba4d_e7b9_45c5_958b_f98022473745);
}
impl ::core::convert::From<IXpsOMPageReferenceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPageReferenceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPageReferenceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPageReferenceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPageReferenceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPageReferenceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, pagereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, pagereference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, pagereference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagereference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPart(pub ::windows::runtime::IUnknown);
impl IXpsOMPart {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPart {
    type Vtable = IXpsOMPart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
}
impl ::core::convert::From<IXpsOMPart> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPart) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPart> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPart) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPartResources(pub ::windows::runtime::IUnknown);
impl IXpsOMPartResources {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFontResources(&self) -> ::windows::runtime::Result<IXpsOMFontResourceCollection> {
        let mut result__: <IXpsOMFontResourceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMFontResourceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetImageResources(&self) -> ::windows::runtime::Result<IXpsOMImageResourceCollection> {
        let mut result__: <IXpsOMImageResourceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMImageResourceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColorProfileResources(&self) -> ::windows::runtime::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__: <IXpsOMColorProfileResourceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMColorProfileResourceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetRemoteDictionaryResources(&self) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__: <IXpsOMRemoteDictionaryResourceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMRemoteDictionaryResourceCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPartResources {
    type Vtable = IXpsOMPartResources_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf4cf7729_4864_4275_99b3_a8717163ecaf);
}
impl ::core::convert::From<IXpsOMPartResources> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPartResources) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPartResources> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPartResources) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPartResources {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPartResources {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResources_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fontresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imageresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorprofileresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionaryresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPartUriCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMPartUriCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPartUriCollection {
    type Vtable = IXpsOMPartUriCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x57c650d4_067c_4893_8c33_f62a0633730f);
}
impl ::core::convert::From<IXpsOMPartUriCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPartUriCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPartUriCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPartUriCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPartUriCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPartUriCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartUriCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPath(pub ::windows::runtime::IUnknown);
impl IXpsOMPath {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::runtime::Result<super::super::System::Com::IUri> {
        let mut result__: <super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGeometryLocal(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetGeometryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometry>>(&self, geometry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), geometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetGeometryLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetGeometryLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, shortdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, longdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSnapsToPixels(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSnapsToPixels<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, snapstopixels: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), snapstopixels.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetStrokeBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetStrokeBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeDashes(&self) -> ::windows::runtime::Result<IXpsOMDashCollection> {
        let mut result__: <IXpsOMDashCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDashCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeDashCap(&self) -> ::windows::runtime::Result<XPS_DASH_CAP> {
        let mut result__: <XPS_DASH_CAP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_DASH_CAP>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashcap)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeDashOffset(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeStartLineCap(&self) -> ::windows::runtime::Result<XPS_LINE_CAP> {
        let mut result__: <XPS_LINE_CAP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_LINE_CAP>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokestartlinecap)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeEndLineCap(&self) -> ::windows::runtime::Result<XPS_LINE_CAP> {
        let mut result__: <XPS_LINE_CAP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_LINE_CAP>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokeendlinecap)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeLineJoin(&self) -> ::windows::runtime::Result<XPS_LINE_JOIN> {
        let mut result__: <XPS_LINE_JOIN as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_LINE_JOIN>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokelinejoin)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeMiterLimit(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokemiterlimit)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetStrokeThickness(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokethickness)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFillBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMPath> {
        let mut result__: <IXpsOMPath as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPath>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPath {
    type Vtable = IXpsOMPath_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x37d38bb6_3ee9_4110_9312_14b194163337);
}
impl ::core::convert::From<IXpsOMPath> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPath) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPath> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPath) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMVisual> for &IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMVisual> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMPath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPath_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, geometry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, longdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, snapstopixels: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokedashes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokedashcap: XPS_DASH_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokedashoffset: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokedashoffset: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokestartlinecap: XPS_LINE_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeendlinecap: XPS_LINE_CAP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokelinejoin: XPS_LINE_JOIN) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokemiterlimit: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokemiterlimit: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokethickness: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokethickness: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMPrintTicketResource(pub ::windows::runtime::IUnknown);
impl IXpsOMPrintTicketResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMPrintTicketResource {
    type Vtable = IXpsOMPrintTicketResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
}
impl ::core::convert::From<IXpsOMPrintTicketResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMPrintTicketResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMPrintTicketResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMPrintTicketResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPrintTicketResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMRadialGradientBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMRadialGradientBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGradientStops(&self) -> ::windows::runtime::Result<IXpsOMGradientStopCollection> {
        let mut result__: <IXpsOMGradientStopCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::runtime::Result<XPS_SPREAD_METHOD> {
        let mut result__: <XPS_SPREAD_METHOD as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::runtime::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: <XPS_COLOR_INTERPOLATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCenter(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(center)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetRadiiSizes(&self) -> ::windows::runtime::Result<XPS_SIZE> {
        let mut result__: <XPS_SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIZE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiisizes)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetGradientOrigin(&self) -> ::windows::runtime::Result<XPS_POINT> {
        let mut result__: <XPS_POINT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_POINT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMRadialGradientBrush> {
        let mut result__: <IXpsOMRadialGradientBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMRadialGradientBrush {
    type Vtable = IXpsOMRadialGradientBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75f207e5_08bf_413c_96b1_b82b4064176b);
}
impl ::core::convert::From<IXpsOMRadialGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMRadialGradientBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMRadialGradientBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMRadialGradientBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMGradientBrush> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMGradientBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMGradientBrush> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMGradientBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRadialGradientBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gradientstops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, center: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, center: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radiisizes: *mut XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radiisizes: *const XPS_SIZE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, origin: *mut XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, origin: *const XPS_POINT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radialgradientbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMRemoteDictionaryResource(pub ::windows::runtime::IUnknown);
impl IXpsOMRemoteDictionaryResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionary<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMRemoteDictionaryResource {
    type Vtable = IXpsOMRemoteDictionaryResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMRemoteDictionaryResource1(pub ::windows::runtime::IUnknown);
impl IXpsOMRemoteDictionaryResource1 {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDictionary(&self) -> ::windows::runtime::Result<IXpsOMDictionary> {
        let mut result__: <IXpsOMDictionary as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDictionary>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetDictionary<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentType(&self) -> ::windows::runtime::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: <XPS_DOCUMENT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn Write1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream>>(&self, stream: Param0, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), stream.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMRemoteDictionaryResource1 {
    type Vtable = IXpsOMRemoteDictionaryResource1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResource1> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResource1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResource1> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResource1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMRemoteDictionaryResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMRemoteDictionaryResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMRemoteDictionaryResourceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, object: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::runtime::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: <IXpsOMRemoteDictionaryResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), &mut result__).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMRemoteDictionaryResourceCollection {
    type Vtable = IXpsOMRemoteDictionaryResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c38db61_7fec_464a_87bd_41e3bef018be);
}
impl ::core::convert::From<IXpsOMRemoteDictionaryResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMRemoteDictionaryResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMRemoteDictionaryResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMRemoteDictionaryResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMRemoteDictionaryResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMRemoteDictionaryResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, remotedictionaryresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMResource(pub ::windows::runtime::IUnknown);
impl IXpsOMResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMResource {
    type Vtable = IXpsOMResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
}
impl ::core::convert::From<IXpsOMResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMShareable(pub ::windows::runtime::IUnknown);
impl IXpsOMShareable {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMShareable {
    type Vtable = IXpsOMShareable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
}
impl ::core::convert::From<IXpsOMShareable> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMShareable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMShareable> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMShareable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMShareable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMShareable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMSignatureBlockResource(pub ::windows::runtime::IUnknown);
impl IXpsOMSignatureBlockResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMDocument> {
        let mut result__: <IXpsOMDocument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMDocument>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMSignatureBlockResource {
    type Vtable = IXpsOMSignatureBlockResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4776ad35_2e04_4357_8743_ebf6c171a905);
}
impl ::core::convert::From<IXpsOMSignatureBlockResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMSignatureBlockResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMSignatureBlockResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMSignatureBlockResourceCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMSignatureBlockResource> {
        let mut result__: <IXpsOMSignatureBlockResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, signatureblockresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), signatureblockresource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::runtime::Result<IXpsOMSignatureBlockResource> {
        let mut result__: <IXpsOMSignatureBlockResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), &mut result__).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMSignatureBlockResourceCollection {
    type Vtable = IXpsOMSignatureBlockResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
}
impl ::core::convert::From<IXpsOMSignatureBlockResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMSignatureBlockResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMSignatureBlockResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMSignatureBlockResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMSignatureBlockResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMSignatureBlockResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signatureblockresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signatureblockresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signatureblockresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureblockresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, signatureblockresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMSolidColorBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMSolidColorBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetColor<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMSolidColorBrush> {
        let mut result__: <IXpsOMSolidColorBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMSolidColorBrush {
    type Vtable = IXpsOMSolidColorBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa06f9f05_3be9_4763_98a8_094fc672e488);
}
impl ::core::convert::From<IXpsOMSolidColorBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMSolidColorBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMSolidColorBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMSolidColorBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *mut XPS_COLOR, colorprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: *const XPS_COLOR, colorprofile: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, solidcolorbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMStoryFragmentsResource(pub ::windows::runtime::IUnknown);
impl IXpsOMStoryFragmentsResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<IXpsOMPageReference> {
        let mut result__: <IXpsOMPageReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMStoryFragmentsResource {
    type Vtable = IXpsOMStoryFragmentsResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc2b3ca09_0473_4282_87ae_1780863223f0);
}
impl ::core::convert::From<IXpsOMStoryFragmentsResource> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMStoryFragmentsResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMStoryFragmentsResource> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMStoryFragmentsResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMResource> for &IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMResource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMPart> for &IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMPart> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMStoryFragmentsResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMThumbnailGenerator(pub ::windows::runtime::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GenerateThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMPage>, Param3: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, page: Param0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: Param3) -> ::windows::runtime::Result<IXpsOMImageResource> {
        let mut result__: <IXpsOMImageResource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(thumbnailtype), ::core::mem::transmute(thumbnailsize), imageresourcepartname.into_param().abi(), &mut result__).from_abi::<IXpsOMImageResource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMThumbnailGenerator {
    type Vtable = IXpsOMThumbnailGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x15b873d5_1971_41e8_83a3_6578403064c7);
}
impl ::core::convert::From<IXpsOMThumbnailGenerator> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMThumbnailGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMThumbnailGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMThumbnailGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMThumbnailGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMThumbnailGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: ::windows::runtime::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::runtime::RawPtr, imageresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMTileBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMTileBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::runtime::Result<XPS_TILE_MODE> {
        let mut result__: <XPS_TILE_MODE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMTileBrush {
    type Vtable = IXpsOMTileBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0fc2328d_d722_4a54_b2ec_be90218a789e);
}
impl ::core::convert::From<IXpsOMTileBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMTileBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMTileBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMTileBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMTileBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMTileBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: *mut XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMVisual(pub ::windows::runtime::IUnknown);
impl IXpsOMVisual {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometry(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::runtime::Result<IXpsOMGeometry> {
        let mut result__: <IXpsOMGeometry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMGeometry>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::runtime::Result<IXpsOMBrush> {
        let mut result__: <IXpsOMBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMBrush>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::runtime::Result<super::super::System::Com::IUri> {
        let mut result__: <super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMVisual {
    type Vtable = IXpsOMVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
}
impl ::core::convert::From<IXpsOMVisual> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMVisual) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMVisual> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisual_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrixtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clipgeometry: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacitymaskbrush: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ishyperlink: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hyperlinkuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMVisualBrush(pub ::windows::runtime::IUnknown);
impl IXpsOMVisualBrush {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOwner(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<XPS_OBJECT_TYPE> {
        let mut result__: <XPS_OBJECT_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetOpacity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransform(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTransformLocal(&self) -> ::windows::runtime::Result<IXpsOMMatrixTransform> {
        let mut result__: <IXpsOMMatrixTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewbox(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetViewport(&self) -> ::windows::runtime::Result<XPS_RECT> {
        let mut result__: <XPS_RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_RECT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetTileMode(&self) -> ::windows::runtime::Result<XPS_TILE_MODE> {
        let mut result__: <XPS_TILE_MODE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_TILE_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetVisual(&self) -> ::windows::runtime::Result<IXpsOMVisual> {
        let mut result__: <IXpsOMVisual as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetVisualLocal(&self) -> ::windows::runtime::Result<IXpsOMVisual> {
        let mut result__: <IXpsOMVisual as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetVisualLocal<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMVisual>>(&self, visual: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetVisualLookup(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetVisualLookup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IXpsOMVisualBrush> {
        let mut result__: <IXpsOMVisualBrush as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsOMVisualBrush>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMVisualBrush {
    type Vtable = IXpsOMVisualBrush_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x97e294af_5b37_46b4_8057_874d2f64119b);
}
impl ::core::convert::From<IXpsOMVisualBrush> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMVisualBrush) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMVisualBrush> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMVisualBrush) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMTileBrush> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMTileBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMTileBrush> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMTileBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMBrush> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMBrush> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXpsOMShareable> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXpsOMShareable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrush_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, owner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, opacity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewbox: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *mut XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *const XPS_RECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: *mut XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tilemode: XPS_TILE_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visual: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visual: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visual: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lookup: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visualbrush: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsOMVisualCollection(pub ::windows::runtime::IUnknown);
impl IXpsOMVisualCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsOMVisual> {
        let mut result__: <IXpsOMVisual as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsOMVisual>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsOMVisual>>(&self, object: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsOMVisualCollection {
    type Vtable = IXpsOMVisualCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
}
impl ::core::convert::From<IXpsOMVisualCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsOMVisualCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsOMVisualCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsOMVisualCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsOMVisualCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsOMVisualCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignature(pub ::windows::runtime::IUnknown);
impl IXpsSignature {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturehashvalue), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__: <super::Packaging::Opc::IOpcCertificateEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcCertificateEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSigningTime(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::runtime::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: <super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::Result<XPS_SIGNATURE_STATUS> {
        let mut result__: <XPS_SIGNATURE_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(x509certificate), &mut result__).from_abi::<XPS_SIGNATURE_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPolicy(&self) -> ::windows::runtime::Result<XPS_SIGN_POLICY> {
        let mut result__: <XPS_SIGN_POLICY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__: <super::Packaging::Opc::IOpcSignatureReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetSignatureXml(&self, signaturexml: *const u8, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignature {
    type Vtable = IXpsSignature_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6ae4c93e_1ade_42fb_898b_3a5658284857);
}
impl ::core::convert::From<IXpsSignature> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignature) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignature> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignature) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignature {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignature {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignature_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificateenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, policy: *mut XPS_SIGN_POLICY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobjectenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturexml: *const u8, count: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureBlock(pub ::windows::runtime::IUnknown);
impl IXpsSignatureBlock {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetRequests(&self) -> ::windows::runtime::Result<IXpsSignatureRequestCollection> {
        let mut result__: <IXpsSignatureRequestCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsSignatureRequestCollection>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetDocumentIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDocumentName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn CreateRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, requestid: Param0) -> ::windows::runtime::Result<IXpsSignatureRequest> {
        let mut result__: <IXpsSignatureRequest as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), requestid.into_param().abi(), &mut result__).from_abi::<IXpsSignatureRequest>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureBlock {
    type Vtable = IXpsSignatureBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x151fac09_0b97_4ac6_a323_5e4297d4322b);
}
impl ::core::convert::From<IXpsSignatureBlock> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureBlock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureBlock> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureBlock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureBlock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureBlock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requests: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fixeddocumentindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fixeddocumentname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureBlockCollection(pub ::windows::runtime::IUnknown);
impl IXpsSignatureBlockCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsSignatureBlock> {
        let mut result__: <IXpsSignatureBlock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsSignatureBlock>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureBlockCollection {
    type Vtable = IXpsSignatureBlockCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23397050_fe99_467a_8dce_9237f074ffe4);
}
impl ::core::convert::From<IXpsSignatureBlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureBlockCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureBlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureBlockCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureBlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureBlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signatureblock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureCollection(pub ::windows::runtime::IUnknown);
impl IXpsSignatureCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsSignature> {
        let mut result__: <IXpsSignature as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsSignature>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureCollection {
    type Vtable = IXpsSignatureCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
}
impl ::core::convert::From<IXpsSignatureCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureManager(pub ::windows::runtime::IUnknown);
impl IXpsSignatureManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn LoadPackageFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn LoadPackageStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Sign<'a, Param0: ::windows::runtime::IntoParam<'a, IXpsSigningOptions>>(&self, signoptions: Param0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::Result<IXpsSignature> {
        let mut result__: <IXpsSignature as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), signoptions.into_param().abi(), ::core::mem::transmute(x509certificate), &mut result__).from_abi::<IXpsSignature>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetSignatureOriginPartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signatureoriginpartname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), signatureoriginpartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignatures(&self) -> ::windows::runtime::Result<IXpsSignatureCollection> {
        let mut result__: <IXpsSignatureCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsSignatureCollection>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn AddSignatureBlock<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0, fixeddocumentindex: u32) -> ::windows::runtime::Result<IXpsSignatureBlock> {
        let mut result__: <IXpsSignatureBlock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(fixeddocumentindex), &mut result__).from_abi::<IXpsSignatureBlock>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignatureBlocks(&self) -> ::windows::runtime::Result<IXpsSignatureBlockCollection> {
        let mut result__: <IXpsSignatureBlockCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsSignatureBlockCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::runtime::Result<IXpsSigningOptions> {
        let mut result__: <IXpsSigningOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsSigningOptions>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn SavePackageToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_System_Com`*"]
    pub unsafe fn SavePackageToStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureManager {
    type Vtable = IXpsSignatureManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
}
impl ::core::convert::From<IXpsSignatureManager> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureManager> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signoptions: ::windows::runtime::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureoriginpartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureoriginpartname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatures: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: ::windows::runtime::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureblocks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signingoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureRequest(pub ::windows::runtime::IUnknown);
impl IXpsSignatureRequest {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetIntent(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetIntent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, intent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), intent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetRequestedSigner(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetRequestedSigner<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetRequestSignByDate(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetRequestSignByDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, datestring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), datestring.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSigningLocale(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSigningLocale<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, place: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), place.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(pagepartname), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetRequestId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetSignature(&self) -> ::windows::runtime::Result<IXpsSignature> {
        let mut result__: <IXpsSignature as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IXpsSignature>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureRequest {
    type Vtable = IXpsSignatureRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
}
impl ::core::convert::From<IXpsSignatureRequest> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureRequest> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, intent: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, intent: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datestring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, place: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, place: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pageindex: *mut i32, pagepartname: *mut ::windows::runtime::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pageindex: i32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSignatureRequestCollection(pub ::windows::runtime::IUnknown);
impl IXpsSignatureRequestCollection {
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXpsSignatureRequest> {
        let mut result__: <IXpsSignatureRequest as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IXpsSignatureRequest>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSignatureRequestCollection {
    type Vtable = IXpsSignatureRequestCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
}
impl ::core::convert::From<IXpsSignatureRequestCollection> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSignatureRequestCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSignatureRequestCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSignatureRequestCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSignatureRequestCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSignatureRequestCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, signaturerequest: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsSigningOptions(pub ::windows::runtime::IUnknown);
impl IXpsSigningOptions {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSignatureId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signatureid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), signatureid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetSignatureMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signaturemethod: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signaturemethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
    pub unsafe fn SetDigestMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, digestmethod: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), digestmethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: <super::Packaging::Opc::IOpcPartUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetSignaturePartName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetPolicy(&self) -> ::windows::runtime::Result<XPS_SIGN_POLICY> {
        let mut result__: <XPS_SIGN_POLICY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::runtime::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: <super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeformat)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomObjects(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__: <super::Packaging::Opc::IOpcSignatureCustomObjectSet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectSet>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomReferences(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__: <super::Packaging::Opc::IOpcSignatureReferenceSet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceSet>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    #[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCertificateSet(&self) -> ::windows::runtime::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__: <super::Packaging::Opc::IOpcCertificateSet as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Packaging::Opc::IOpcCertificateSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<XPS_SIGN_FLAGS> {
        let mut result__: <XPS_SIGN_FLAGS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_SIGN_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps`*"]
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsSigningOptions {
    type Vtable = IXpsSigningOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7718eae4_3215_49be_af5b_594fef7fcfa6);
}
impl ::core::convert::From<IXpsSigningOptions> for ::windows::runtime::IUnknown {
    fn from(value: IXpsSigningOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsSigningOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsSigningOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsSigningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsSigningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSigningOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, policy: *mut XPS_SIGN_POLICY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, policy: XPS_SIGN_POLICY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobjectset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customreferenceset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificateset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: *mut XPS_SIGN_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: XPS_SIGN_FLAGS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
impl ::core::convert::From<u32> for PRINT_WINDOW_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRINT_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for PRINT_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PRINT_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PRINT_WINDOW_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PRINT_WINDOW_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PRINT_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl PSFEATURE_CUSTPAPER {}
impl ::core::default::Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PSFEATURE_CUSTPAPER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PSFEATURE_CUSTPAPER").field("lOrientation", &self.lOrientation).field("lWidth", &self.lWidth).field("lHeight", &self.lHeight).field("lWidthOffset", &self.lWidthOffset).field("lHeightOffset", &self.lHeightOffset).finish()
    }
}
impl ::core::cmp::PartialEq for PSFEATURE_CUSTPAPER {
    fn eq(&self, other: &Self) -> bool {
        self.lOrientation == other.lOrientation && self.lWidth == other.lWidth && self.lHeight == other.lHeight && self.lWidthOffset == other.lWidthOffset && self.lHeightOffset == other.lHeightOffset
    }
}
impl ::core::cmp::Eq for PSFEATURE_CUSTPAPER {}
unsafe impl ::windows::runtime::Abi for PSFEATURE_CUSTPAPER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`*"]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSFEATURE_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PSFEATURE_OUTPUT").field("bPageIndependent", &self.bPageIndependent).field("bSetPageDevice", &self.bSetPageDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSFEATURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.bPageIndependent == other.bPageIndependent && self.bSetPageDevice == other.bSetPageDevice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSFEATURE_OUTPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl PSINJECTDATA {}
impl ::core::default::Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PSINJECTDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PSINJECTDATA").field("DataBytes", &self.DataBytes).field("InjectionPoint", &self.InjectionPoint).field("PageNumber", &self.PageNumber).finish()
    }
}
impl ::core::cmp::PartialEq for PSINJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytes == other.DataBytes && self.InjectionPoint == other.InjectionPoint && self.PageNumber == other.PageNumber
    }
}
impl ::core::cmp::Eq for PSINJECTDATA {}
unsafe impl ::windows::runtime::Abi for PSINJECTDATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSINJECT_POINT(pub u16);
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
impl ::core::convert::From<u16> for PSINJECT_POINT {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSINJECT_POINT {
    type Abi = Self;
}
impl ::core::ops::BitOr for PSINJECT_POINT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PSINJECT_POINT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PSINJECT_POINT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PSINJECT_POINT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PSINJECT_POINT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdcblt: Param1, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetAbortProc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, proc: ::core::option::Option<ABORTPROC>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAbortProc(hdc: super::super::Graphics::Gdi::HDC, proc: ::windows::runtime::RawPtr) -> i32;
        }
        ::core::mem::transmute(SetAbortProc(hdc.into_param().abi(), ::core::mem::transmute(proc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn StartDocA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpdi: *const DOCINFOA) -> i32 {
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
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn StartDocW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpdi: *const DOCINFOW) -> i32 {
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
#[doc = "*Required features: `Win32_Storage_Xps`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) -> i32 {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl XPS_COLOR {}
impl ::core::default::Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_COLOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for XPS_COLOR {}
unsafe impl ::windows::runtime::Abi for XPS_COLOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl XPS_COLOR_0 {}
impl ::core::default::Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_COLOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0 {}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl XPS_COLOR_0_0 {}
impl ::core::default::Default for XPS_COLOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_context_e__Struct").field("channelCount", &self.channelCount).field("channels", &self.channels).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.channelCount == other.channelCount && self.channels == other.channels
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_0 {}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl XPS_COLOR_0_1 {}
impl ::core::default::Default for XPS_COLOR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_sRGB_e__Struct").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.alpha == other.alpha && self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_1 {}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl XPS_COLOR_0_2 {}
impl ::core::default::Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_COLOR_0_2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_scRGB_e__Struct").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.alpha == other.alpha && self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_2 {}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_0_2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
impl ::core::convert::From<i32> for XPS_COLOR_INTERPOLATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_INTERPOLATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_COLOR_TYPE(pub i32);
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
impl ::core::convert::From<i32> for XPS_COLOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_COLOR_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl XPS_DASH {}
impl ::core::default::Default for XPS_DASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_DASH {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_DASH").field("length", &self.length).field("gap", &self.gap).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_DASH {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.gap == other.gap
    }
}
impl ::core::cmp::Eq for XPS_DASH {}
unsafe impl ::windows::runtime::Abi for XPS_DASH {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_DASH_CAP(pub i32);
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
impl ::core::convert::From<i32> for XPS_DASH_CAP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_DASH_CAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
impl ::core::convert::From<i32> for XPS_DOCUMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_DOCUMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108159i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_ALREADY_OWNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108413i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108407i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108409i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108408i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108923i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108922i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108410i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108671i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_DUPLICATE_NAMES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109175i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109184i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108416i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_BLEED_BOX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109692i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_CONTENT_BOX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109685i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109682i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_FLOAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109689i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_FONT_URI: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109686i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_LANGUAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109696i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109690i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_MARKUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109684i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109695i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108158i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108160i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109681i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_PAGE_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109693i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109694i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108789i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109691i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_INVALID_XML_ENCODING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109683i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108924i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108925i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108926i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108791i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_COLORPROFILE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109436i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109422i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109431i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109432i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_FONTURI: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109433i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_GLYPHS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109438i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109426i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_LOOKUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109439i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109440i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109428i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109427i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_PART_REFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109424i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_PART_STREAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109421i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109430i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109429i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109435i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109425i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109434i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109423i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109437i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109182i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109178i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109177i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109179i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109176i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_RESOURCES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109183i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109180i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109181i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_NEGATIVE_FLOAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108918i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108670i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108405i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108414i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_OBJECT_DETACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108790i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_ODD_BIDILEVEL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108921i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108920i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108793i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108794i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108404i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108406i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108412i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108919i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_SIGNATUREID_DUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108792i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_SIGREQUESTID_DUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108795i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_STRING_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108928i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_TOO_MANY_INDICES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108927i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109420i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108411i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109688i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109680i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142109679i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108415i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142108672i32 as _);
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_FILL_RULE(pub i32);
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
impl ::core::convert::From<i32> for XPS_FILL_RULE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_FILL_RULE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_FONT_EMBEDDING(pub i32);
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
impl ::core::convert::From<i32> for XPS_FONT_EMBEDDING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_FONT_EMBEDDING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl XPS_GLYPH_INDEX {}
impl ::core::default::Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_GLYPH_INDEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_GLYPH_INDEX").field("index", &self.index).field("advanceWidth", &self.advanceWidth).field("horizontalOffset", &self.horizontalOffset).field("verticalOffset", &self.verticalOffset).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_GLYPH_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.advanceWidth == other.advanceWidth && self.horizontalOffset == other.horizontalOffset && self.verticalOffset == other.verticalOffset
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_INDEX {}
unsafe impl ::windows::runtime::Abi for XPS_GLYPH_INDEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl XPS_GLYPH_MAPPING {}
impl ::core::default::Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_GLYPH_MAPPING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_GLYPH_MAPPING").field("unicodeStringStart", &self.unicodeStringStart).field("unicodeStringLength", &self.unicodeStringLength).field("glyphIndicesStart", &self.glyphIndicesStart).field("glyphIndicesLength", &self.glyphIndicesLength).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_GLYPH_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.unicodeStringStart == other.unicodeStringStart && self.unicodeStringLength == other.unicodeStringLength && self.glyphIndicesStart == other.glyphIndicesStart && self.glyphIndicesLength == other.glyphIndicesLength
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_MAPPING {}
unsafe impl ::windows::runtime::Abi for XPS_GLYPH_MAPPING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_IMAGE_TYPE(pub i32);
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
impl ::core::convert::From<i32> for XPS_IMAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_IMAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_INTERLEAVING(pub i32);
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
impl ::core::convert::From<i32> for XPS_INTERLEAVING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_INTERLEAVING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_LINE_CAP(pub i32);
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
impl ::core::convert::From<i32> for XPS_LINE_CAP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_LINE_CAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_LINE_JOIN(pub i32);
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
impl ::core::convert::From<i32> for XPS_LINE_JOIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_LINE_JOIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl XPS_MATRIX {}
impl ::core::default::Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_MATRIX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("m31", &self.m31).field("m32", &self.m32).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.m31 == other.m31 && self.m32 == other.m32
    }
}
impl ::core::cmp::Eq for XPS_MATRIX {}
unsafe impl ::windows::runtime::Abi for XPS_MATRIX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_OBJECT_TYPE(pub i32);
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
impl ::core::convert::From<i32> for XPS_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl XPS_POINT {}
impl ::core::default::Default for XPS_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_POINT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_POINT").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for XPS_POINT {}
unsafe impl ::windows::runtime::Abi for XPS_POINT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl XPS_RECT {}
impl ::core::default::Default for XPS_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_RECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_RECT").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_RECT {}
unsafe impl ::windows::runtime::Abi for XPS_RECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
impl ::core::convert::From<i32> for XPS_SEGMENT_STROKE_PATTERN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SEGMENT_STROKE_PATTERN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SEGMENT_TYPE(pub i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
impl ::core::convert::From<i32> for XPS_SEGMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SEGMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
impl ::core::convert::From<i32> for XPS_SIGNATURE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SIGNATURE_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SIGN_FLAGS(pub i32);
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
impl ::core::convert::From<i32> for XPS_SIGN_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SIGN_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SIGN_POLICY(pub i32);
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
impl ::core::convert::From<i32> for XPS_SIGN_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SIGN_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps`*"]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl XPS_SIZE {}
impl ::core::default::Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_SIZE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_SIZE").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_SIZE {}
unsafe impl ::windows::runtime::Abi for XPS_SIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_SPREAD_METHOD(pub i32);
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
impl ::core::convert::From<i32> for XPS_SPREAD_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_SPREAD_METHOD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_STYLE_SIMULATION(pub i32);
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
impl ::core::convert::From<i32> for XPS_STYLE_SIMULATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_STYLE_SIMULATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
impl ::core::convert::From<i32> for XPS_THUMBNAIL_SIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_THUMBNAIL_SIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Xps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_TILE_MODE(pub i32);
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
impl ::core::convert::From<i32> for XPS_TILE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_TILE_MODE {
    type Abi = Self;
}
pub const XpsOMObjectFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);

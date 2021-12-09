#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
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
pub type DEVICE_CAPABILITIES = u32;
pub const DC_BINNAMES: DEVICE_CAPABILITIES = 12u32;
pub const DC_BINS: DEVICE_CAPABILITIES = 6u32;
pub const DC_COLLATE: DEVICE_CAPABILITIES = 22u32;
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = 32u32;
pub const DC_COPIES: DEVICE_CAPABILITIES = 18u32;
pub const DC_DRIVER: DEVICE_CAPABILITIES = 11u32;
pub const DC_DUPLEX: DEVICE_CAPABILITIES = 7u32;
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = 13u32;
pub const DC_EXTRA: DEVICE_CAPABILITIES = 9u32;
pub const DC_FIELDS: DEVICE_CAPABILITIES = 1u32;
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = 14u32;
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = 5u32;
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = 29u32;
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = 34u32;
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = 35u32;
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = 4u32;
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = 17u32;
pub const DC_NUP: DEVICE_CAPABILITIES = 33u32;
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = 16u32;
pub const DC_PAPERS: DEVICE_CAPABILITIES = 2u32;
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = 3u32;
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = 25u32;
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = 28u32;
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = 26u32;
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = 31u32;
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = 27u32;
pub const DC_SIZE: DEVICE_CAPABILITIES = 8u32;
pub const DC_STAPLE: DEVICE_CAPABILITIES = 30u32;
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = 15u32;
pub const DC_VERSION: DEVICE_CAPABILITIES = 10u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PSTR,
    pub lpszOutput: super::super::Foundation::PSTR,
    pub lpszDatatype: super::super::Foundation::PSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOCINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOCINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOCINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOCINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOCINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PWSTR,
    pub lpszOutput: super::super::Foundation::PWSTR,
    pub lpszDatatype: super::super::Foundation::PWSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOCINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOCINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOCINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOCINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOCINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32 {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pdevice: Param0, pport: Param1, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32 {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn Escape<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdc: Param0, iescape: i32, cjin: i32, pvin: Param3, pvout: *mut ::core::ffi::c_void) -> i32 {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ExtEscape<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdc: Param0, iescape: i32, cjinput: i32, lpindata: Param3, cjoutput: i32, lpoutdata: super::super::Foundation::PSTR) -> i32 {
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
pub type HPTPROVIDER = isize;
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetXpsOMPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, discardcontrolpartname: Param1) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMObjectFactory>(result__)
    }
    pub unsafe fn GetXpsType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsDocumentPackageTarget {
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
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget {
    type Vtable = IXpsDocumentPackageTargetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0b6d38_53ad_41da_b212_d37637a6714e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTargetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget3D(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter3D<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, documentsequencepartname: Param0, discardcontrolpartname: Param1, modelpartname: Param2, modeldata: Param3) -> ::windows::core::Result<IXpsOMPackageWriter3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), modelpartname.into_param().abi(), modeldata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter3D>(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMObjectFactory>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsDocumentPackageTarget3D {
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
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget3D {
    type Vtable = IXpsDocumentPackageTarget3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60ba71b8_3101_4984_9199_f4ea775ff01d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMBrush(::windows::core::IUnknown);
impl IXpsOMBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMBrush {
    type Vtable = IXpsOMBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMCanvas(::windows::core::IUnknown);
impl IXpsOMCanvas {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseAliasedEdgeMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, usealiasededgemode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), usealiasededgemode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, shortdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, longdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMCanvas {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMCanvas {
    type Vtable = IXpsOMCanvasVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvasVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMColorProfileResource(::windows::core::IUnknown);
impl IXpsOMColorProfileResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMColorProfileResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResource {
    type Vtable = IXpsOMColorProfileResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMColorProfileResourceCollection(::windows::core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMColorProfileResourceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResourceCollection {
    type Vtable = IXpsOMColorProfileResourceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12759630_5fba_4283_8f7d_cca849809edb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMCoreProperties(::windows::core::IUnknown);
impl IXpsOMCoreProperties {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCategory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, category: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), category.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentStatus(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, contentstatus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), contentstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, contenttype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), contenttype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreated(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(created)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreator(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, creator: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), creator.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIdentifier<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, identifier: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), identifier.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKeywords(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKeywords<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, keywords: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), keywords.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastModifiedBy(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastModifiedBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lastmodifiedby: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), lastmodifiedby.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastPrinted(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(lastprinted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(modified)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRevision<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, revision: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), revision.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSubject(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, subject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), subject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTitle(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, title: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), title.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMCoreProperties {
    type Vtable = IXpsOMCorePropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCorePropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDashCollection(::windows::core::IUnknown);
impl IXpsOMDashCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<XPS_DASH> {
        let mut result__: XPS_DASH = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DASH>(result__)
    }
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(dash)).ok()
    }
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dash)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDashCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMDashCollection {
    type Vtable = IXpsOMDashCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDictionary(::windows::core::IUnknown);
impl IXpsOMDictionary {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAt(&self, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(key), ::core::mem::transmute(entry)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetByKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, beforeentry: Param1) -> ::windows::core::Result<IXpsOMShareable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), key.into_param().abi(), beforeentry.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMShareable>(result__)
    }
    pub unsafe fn GetIndex<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, entry: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), entry.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, key: Param0, entry: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IXpsOMShareable>>(&self, index: u32, key: Param1, entry: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDictionary {
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
unsafe impl ::windows::core::Interface for IXpsOMDictionary {
    type Vtable = IXpsOMDictionaryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDictionaryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDocument(::windows::core::IUnknown);
impl IXpsOMDocument {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    pub unsafe fn GetPageReferences(&self) -> ::windows::core::Result<IXpsOMPageReferenceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReferenceCollection>(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentStructureResource(&self) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    pub unsafe fn SetDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>>(&self, documentstructureresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), documentstructureresource.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureBlockResources(&self) -> ::windows::core::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResourceCollection>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMDocument {
    type Vtable = IXpsOMDocumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDocumentCollection(::windows::core::IUnknown);
impl IXpsOMDocumentCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, index: u32, document: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), document.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocument>>(&self, document: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), document.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDocumentCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMDocumentCollection {
    type Vtable = IXpsOMDocumentCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1c87f0d_e947_4754_8a25_971478f7e83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDocumentSequence(::windows::core::IUnknown);
impl IXpsOMDocumentSequence {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<IXpsOMDocumentCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentCollection>(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDocumentSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMDocumentSequence {
    type Vtable = IXpsOMDocumentSequenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequenceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMDocumentStructureResource(::windows::core::IUnknown);
impl IXpsOMDocumentStructureResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMDocumentStructureResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMDocumentStructureResource {
    type Vtable = IXpsOMDocumentStructureResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85febc8a_6b63_48a9_af07_7064e4ecff30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentStructureResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMFontResource(::windows::core::IUnknown);
impl IXpsOMFontResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, embeddingoption: XPS_FONT_EMBEDDING, partname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(embeddingoption), partname.into_param().abi()).ok()
    }
    pub unsafe fn GetEmbeddingOption(&self) -> ::windows::core::Result<XPS_FONT_EMBEDDING> {
        let mut result__: XPS_FONT_EMBEDDING = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_FONT_EMBEDDING>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMFontResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMFontResource {
    type Vtable = IXpsOMFontResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMFontResourceCollection(::windows::core::IUnknown);
impl IXpsOMFontResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMFontResourceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMFontResourceCollection {
    type Vtable = IXpsOMFontResourceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMGeometry(::windows::core::IUnknown);
impl IXpsOMGeometry {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetFigures(&self) -> ::windows::core::Result<IXpsOMGeometryFigureCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigureCollection>(result__)
    }
    pub unsafe fn GetFillRule(&self) -> ::windows::core::Result<XPS_FILL_RULE> {
        let mut result__: XPS_FILL_RULE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_FILL_RULE>(result__)
    }
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fillrule)).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGeometry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMGeometry {
    type Vtable = IXpsOMGeometryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGeometryFigure(::windows::core::IUnknown);
impl IXpsOMGeometryFigure {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(datacount), ::core::mem::transmute(segmentdata)).ok()
    }
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmenttypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentstrokes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(segmentcount), ::core::mem::transmute(segmentdatacount), ::core::mem::transmute(segmenttypes), ::core::mem::transmute(segmentdata), ::core::mem::transmute(segmentstrokes)).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isclosed: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), isclosed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsFilled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsFilled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isfilled: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), isfilled.into_param().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSegmentDataCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSegmentStrokePattern(&self) -> ::windows::core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__: XPS_SEGMENT_STROKE_PATTERN = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SEGMENT_STROKE_PATTERN>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGeometryFigure {
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
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigure {
    type Vtable = IXpsOMGeometryFigureVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd410dc83_908c_443e_8947_b1795d3c165a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGeometryFigureCollection(::windows::core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, index: u32, geometryfigure: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), geometryfigure.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometryFigure>>(&self, geometryfigure: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), geometryfigure.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGeometryFigureCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigureCollection {
    type Vtable = IXpsOMGeometryFigureCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGlyphs(::windows::core::IUnknown);
impl IXpsOMGlyphs {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(prohibitedcaretstopcount), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetStyleSimulations(&self) -> ::windows::core::Result<XPS_STYLE_SIMULATION> {
        let mut result__: XPS_STYLE_SIMULATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_STYLE_SIMULATION>(result__)
    }
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(stylesimulations)).ok()
    }
    pub unsafe fn GetOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    pub unsafe fn GetFontRenderingEmSize(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontrenderingemsize)).ok()
    }
    pub unsafe fn GetFontResource(&self) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    pub unsafe fn SetFontResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), fontresource.into_param().abi()).ok()
    }
    pub unsafe fn GetFontFaceIndex(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(fontfaceindex)).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, fillbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), fillbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetGlyphsEditor(&self) -> ::windows::core::Result<IXpsOMGlyphsEditor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphsEditor>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMGlyphs {
    type Vtable = IXpsOMGlyphsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGlyphsEditor(::windows::core::IUnknown);
impl IXpsOMGlyphsEditor {
    pub unsafe fn ApplyEdits(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnicodeString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, unicodestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), unicodestring.into_param().abi()).ok()
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(glyphindices)).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(glyphmappingcount), ::core::mem::transmute(glyphmappings)).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prohibitedcaretstops)).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bidilevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsSideways<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, issideways: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), issideways.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeviceFontName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, devicefontname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), devicefontname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGlyphsEditor {
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
unsafe impl ::windows::core::Interface for IXpsOMGlyphsEditor {
    type Vtable = IXpsOMGlyphsEditorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsEditorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMGradientBrush(::windows::core::IUnknown);
impl IXpsOMGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMGradientBrush {
    type Vtable = IXpsOMGradientBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedb59622_61a2_42c3_bace_acf2286c06bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGradientStop(::windows::core::IUnknown);
impl IXpsOMGradientStop {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientBrush>(result__)
    }
    pub unsafe fn GetOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOffset(&self, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGradientStop {
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
unsafe impl ::windows::core::Interface for IXpsOMGradientStop {
    type Vtable = IXpsOMGradientStopVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMGradientStopCollection(::windows::core::IUnknown);
impl IXpsOMGradientStopCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, index: u32, stop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), stop.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, stop: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), stop.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMGradientStopCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMGradientStopCollection {
    type Vtable = IXpsOMGradientStopCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMImageBrush(::windows::core::IUnknown);
impl IXpsOMImageBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    pub unsafe fn GetImageResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    pub unsafe fn SetImageResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn GetColorProfileResource(&self) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    pub unsafe fn SetColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, colorprofileresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), colorprofileresource.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMImageBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMImageBrush {
    type Vtable = IXpsOMImageBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df0b466_d382_49ef_8550_dd94c80242e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMImageResource(::windows::core::IUnknown);
impl IXpsOMImageResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, imagetype: XPS_IMAGE_TYPE, partname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), ::core::mem::transmute(imagetype), partname.into_param().abi()).ok()
    }
    pub unsafe fn GetImageType(&self) -> ::windows::core::Result<XPS_IMAGE_TYPE> {
        let mut result__: XPS_IMAGE_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_IMAGE_TYPE>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMImageResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMImageResource {
    type Vtable = IXpsOMImageResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3db8417d_ae50_485e_9a44_d7758f78a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMImageResourceCollection(::windows::core::IUnknown);
impl IXpsOMImageResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMImageResourceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMImageResourceCollection {
    type Vtable = IXpsOMImageResourceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMLinearGradientBrush(::windows::core::IUnknown);
impl IXpsOMLinearGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint)).ok()
    }
    pub unsafe fn GetEndPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(endpoint)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMLinearGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMLinearGradientBrush {
    type Vtable = IXpsOMLinearGradientBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005e279f_c30d_40ff_93ec_1950d3c528db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMMatrixTransform(::windows::core::IUnknown);
impl IXpsOMMatrixTransform {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetMatrix(&self) -> ::windows::core::Result<XPS_MATRIX> {
        let mut result__: XPS_MATRIX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_MATRIX>(result__)
    }
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMMatrixTransform {
    type Vtable = IXpsOMMatrixTransformVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransformVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMNameCollection(::windows::core::IUnknown);
impl IXpsOMNameCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMNameCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMNameCollection {
    type Vtable = IXpsOMNameCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bddf8ec_c915_421b_a166_d173d25653d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMObjectFactory(::windows::core::IUnknown);
impl IXpsOMObjectFactory {
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePage<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
    }
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePackageWriterOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
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
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
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
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uri: Param0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), uri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMObjectFactory {
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
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory {
    type Vtable = IXpsOMObjectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMObjectFactory1(::windows::core::IUnknown);
impl IXpsOMObjectFactory1 {
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentStructureResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateRemoteDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, dictionary: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), dictionary.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, dictionaryparturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(advisorypagedimensions), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePage<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCanvas>(result__)
    }
    pub unsafe fn CreateGlyphs<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMFontResource>>(&self, fontresource: Param0) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fontresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGlyphs>(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(startpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometryFigure>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn CreateSolidColorBrush<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResource>(result__)
    }
    pub unsafe fn CreateImageBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, image: Param0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), image.into_param().abi(), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageBrush>(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox), ::core::mem::transmute(viewport), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, contenttype: XPS_IMAGE_TYPE, parturi: Param2) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(contenttype), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, acquiredstream: Param0, parturi: Param1) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, acquiredstream: Param0, fontembedding: XPS_FONT_EMBEDDING, parturi: Param2, isobfsourcestream: Param3) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), acquiredstream.into_param().abi(), ::core::mem::transmute(fontembedding), parturi.into_param().abi(), isobfsourcestream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResource>(result__)
    }
    pub unsafe fn CreateGradientStop<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStop>(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(startpoint), ::core::mem::transmute(endpoint), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMLinearGradientBrush>(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGradientStop>, Param1: ::windows::core::IntoParam<'a, IXpsOMGradientStop>>(&self, gradstop1: Param0, gradstop2: Param1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), ::core::mem::transmute(centerpoint), ::core::mem::transmute(gradientorigin), ::core::mem::transmute(radiisizes), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartUriCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePackageWriterOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
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
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
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
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uri: Param0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), uri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateReadOnlyStreamOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentTypeFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, xpsdocumentstream: Param0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), xpsdocumentstream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    pub unsafe fn ConvertHDPhotoToJpegXR<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn ConvertJpegXRToHDPhoto<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePackageWriterOnFile1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param6: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>, Param7: ::windows::core::IntoParam<'a, IXpsOMImageResource>, Param8: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param9: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(
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
        (::windows::core::Interface::vtable(self).44)(
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
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(interleaving), documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(documenttype), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
    }
    pub unsafe fn CreatePackage1(&self) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), stream.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, reuseobjects: Param1) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), filename.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
    pub unsafe fn CreatePage1<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, pagedimensions: *const XPS_SIZE, language: Param1, parturi: Param2) -> ::windows::core::Result<IXpsOMPage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions), language.into_param().abi(), parturi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pagemarkupstream: Param0, parturi: Param1, resources: Param2, reuseobjects: Param3) -> ::windows::core::Result<IXpsOMPage1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage1>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartResources>>(&self, dictionarymarkupstream: Param0, parturi: Param1, resources: Param2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), dictionarymarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMObjectFactory> for &IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMObjectFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMObjectFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory1 {
    type Vtable = IXpsOMObjectFactory1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPackage(::windows::core::IUnknown);
impl IXpsOMPackage {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPackage {
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
unsafe impl ::windows::core::Interface for IXpsOMPackage {
    type Vtable = IXpsOMPackageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18c3df65_81e1_4674_91dc_fc452f5a416f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPackage1(::windows::core::IUnknown);
impl IXpsOMPackage1 {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocumentSequence>(result__)
    }
    pub unsafe fn SetDocumentSequence<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDocumentSequence>>(&self, documentsequence: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequence.into_param().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMCoreProperties>(result__)
    }
    pub unsafe fn SetCoreProperties<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMCoreProperties>>(&self, coreproperties: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), coreproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetDiscardControlPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, discardcontrolparturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), discardcontrolparturi.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: Param3, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, outputstream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackage> for &IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPackage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPackage1 {
    type Vtable = IXpsOMPackage1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPackageTarget(::windows::core::IUnknown);
impl IXpsOMPackageTarget {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn CreateXpsOMPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, documentsequencepartname: Param0, documentsequenceprintticket: Param1, discardcontrolpartname: Param2) -> ::windows::core::Result<IXpsOMPackageWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequencepartname.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPackageWriter>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPackageTarget {
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
unsafe impl ::windows::core::Interface for IXpsOMPackageTarget {
    type Vtable = IXpsOMPackageTargetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTargetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPackageWriter(::windows::core::IUnknown);
impl IXpsOMPackageWriter {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>>(&self, documentpartname: Param0, documentprintticket: Param1, documentstructure: Param2, signatureblockresources: Param3, restrictedfonts: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    pub unsafe fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, page: Param0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: Param2, storyfragments: Param3, pageprintticket: Param4, pagethumbnail: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPackageWriter {
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
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter {
    type Vtable = IXpsOMPackageWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPackageWriter3D(::windows::core::IUnknown);
impl IXpsOMPackageWriter3D {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn StartNewDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param2: ::windows::core::IntoParam<'a, IXpsOMDocumentStructureResource>, Param3: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResourceCollection>, Param4: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>>(&self, documentpartname: Param0, documentprintticket: Param1, documentstructure: Param2, signatureblockresources: Param3, restrictedfonts: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    pub unsafe fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param2: ::windows::core::IntoParam<'a, IXpsOMPartUriCollection>, Param3: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>, Param4: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>, Param5: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, page: Param0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: Param2, storyfragments: Param3, pageprintticket: Param4, pagethumbnail: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(advisorypagedimensions), discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    pub unsafe fn AddResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMResource>>(&self, resource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), resource.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddModelTexture<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, texturepartname: Param0, texturedata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), texturepartname.into_param().abi(), texturedata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, printticketpartname: Param0, printticketdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), printticketpartname.into_param().abi(), printticketdata.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPackageWriter> for &IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPackageWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPackageWriter3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter3D {
    type Vtable = IXpsOMPackageWriter3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPage(::windows::core::IUnknown);
impl IXpsOMPage {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPage {
    type Vtable = IXpsOMPageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e18888_f120_4fee_8c68_35296eae91d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMPage1(::windows::core::IUnknown);
impl IXpsOMPage1 {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualCollection>(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentbox)).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bleedbox)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlinktarget: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ishyperlinktarget.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn SetDictionaryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, resourcedictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn SetDictionaryResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, remotedictionaryresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stream: Param0, optimizemarkupsize: Param1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPage> for &IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPage> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPage1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPage1 {
    type Vtable = IXpsOMPage1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x305b60ef_6892_4dda_9cbb_3aa65974508a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPageReference(::windows::core::IUnknown);
impl IXpsOMPageReference {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    pub unsafe fn GetPage(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPage>(result__)
    }
    pub unsafe fn SetPage<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>>(&self, page: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), page.into_param().abi()).ok()
    }
    pub unsafe fn DiscardPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPageLoaded(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagedimensions)).ok()
    }
    pub unsafe fn GetStoryFragmentsResource(&self) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMStoryFragmentsResource>(result__)
    }
    pub unsafe fn SetStoryFragmentsResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMStoryFragmentsResource>>(&self, storyfragmentsresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), storyfragmentsresource.into_param().abi()).ok()
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPrintTicketResource>(result__)
    }
    pub unsafe fn SetPrintTicketResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPrintTicketResource>>(&self, printticketresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), printticketresource.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
    }
    pub unsafe fn SetThumbnailResource<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMImageResource>>(&self, imageresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn CollectLinkTargets(&self) -> ::windows::core::Result<IXpsOMNameCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMNameCollection>(result__)
    }
    pub unsafe fn CollectPartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPartResources>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasRestrictedFonts(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPageReference {
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
unsafe impl ::windows::core::Interface for IXpsOMPageReference {
    type Vtable = IXpsOMPageReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed360180_6f92_4998_890d_2f208531a0a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMPageReferenceCollection(::windows::core::IUnknown);
impl IXpsOMPageReferenceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, index: u32, pagereference: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), pagereference.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPageReference>>(&self, pagereference: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pagereference.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPageReferenceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMPageReferenceCollection {
    type Vtable = IXpsOMPageReferenceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca16ba4d_e7b9_45c5_958b_f98022473745);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMPart(::windows::core::IUnknown);
impl IXpsOMPart {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPart {
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
unsafe impl ::windows::core::Interface for IXpsOMPart {
    type Vtable = IXpsOMPartVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPartResources(::windows::core::IUnknown);
impl IXpsOMPartResources {
    pub unsafe fn GetFontResources(&self) -> ::windows::core::Result<IXpsOMFontResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMFontResourceCollection>(result__)
    }
    pub unsafe fn GetImageResources(&self) -> ::windows::core::Result<IXpsOMImageResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResourceCollection>(result__)
    }
    pub unsafe fn GetColorProfileResources(&self) -> ::windows::core::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMColorProfileResourceCollection>(result__)
    }
    pub unsafe fn GetRemoteDictionaryResources(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResourceCollection>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPartResources {
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
unsafe impl ::windows::core::Interface for IXpsOMPartResources {
    type Vtable = IXpsOMPartResourcesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4cf7729_4864_4275_99b3_a8717163ecaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResourcesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMPartUriCollection(::windows::core::IUnknown);
impl IXpsOMPartUriCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, index: u32, parturi: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPartUriCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMPartUriCollection {
    type Vtable = IXpsOMPartUriCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57c650d4_067c_4893_8c33_f62a0633730f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartUriCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMPath(::windows::core::IUnknown);
impl IXpsOMPath {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn SetGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, geometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), geometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGeometryLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccessibilityShortDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, shortdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), shortdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccessibilityLongDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, longdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), longdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapsToPixels(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSnapsToPixels<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, snapstopixels: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), snapstopixels.into_param().abi()).ok()
    }
    pub unsafe fn GetStrokeBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetStrokeBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetStrokeBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrokeBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrokeBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn GetStrokeDashes(&self) -> ::windows::core::Result<IXpsOMDashCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDashCollection>(result__)
    }
    pub unsafe fn GetStrokeDashCap(&self) -> ::windows::core::Result<XPS_DASH_CAP> {
        let mut result__: XPS_DASH_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DASH_CAP>(result__)
    }
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashcap)).ok()
    }
    pub unsafe fn GetStrokeDashOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokedashoffset)).ok()
    }
    pub unsafe fn GetStrokeStartLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__: XPS_LINE_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_CAP>(result__)
    }
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokestartlinecap)).ok()
    }
    pub unsafe fn GetStrokeEndLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__: XPS_LINE_CAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_CAP>(result__)
    }
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokeendlinecap)).ok()
    }
    pub unsafe fn GetStrokeLineJoin(&self) -> ::windows::core::Result<XPS_LINE_JOIN> {
        let mut result__: XPS_LINE_JOIN = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_LINE_JOIN>(result__)
    }
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokelinejoin)).ok()
    }
    pub unsafe fn GetStrokeMiterLimit(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokemiterlimit)).ok()
    }
    pub unsafe fn GetStrokeThickness(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(strokethickness)).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetFillBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, brush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), brush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFillBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPath>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMVisual> for &IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMVisual> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPath {
    type Vtable = IXpsOMPathVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37d38bb6_3ee9_4110_9312_14b194163337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPathVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMPrintTicketResource(::windows::core::IUnknown);
impl IXpsOMPrintTicketResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMPrintTicketResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMPrintTicketResource {
    type Vtable = IXpsOMPrintTicketResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPrintTicketResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMRadialGradientBrush(::windows::core::IUnknown);
impl IXpsOMRadialGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGradientStopCollection>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__: XPS_SPREAD_METHOD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SPREAD_METHOD>(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(spreadmethod)).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__: XPS_COLOR_INTERPOLATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_COLOR_INTERPOLATION>(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorinterpolationmode)).ok()
    }
    pub unsafe fn GetCenter(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(center)).ok()
    }
    pub unsafe fn GetRadiiSizes(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__: XPS_SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIZE>(result__)
    }
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiisizes)).ok()
    }
    pub unsafe fn GetGradientOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__: XPS_POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_POINT>(result__)
    }
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(origin)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRadialGradientBrush>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMGradientBrush> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMGradientBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMRadialGradientBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMRadialGradientBrush {
    type Vtable = IXpsOMRadialGradientBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f207e5_08bf_413c_96b1_b82b4064176b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRadialGradientBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn SetDictionary<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMRemoteDictionaryResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource {
    type Vtable = IXpsOMRemoteDictionaryResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource1(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource1 {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDictionary>(result__)
    }
    pub unsafe fn SetDictionary<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMDictionary>>(&self, dictionary: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__: XPS_DOCUMENT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_DOCUMENT_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream>>(&self, stream: Param0, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), stream.into_param().abi(), ::core::mem::transmute(documenttype)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMRemoteDictionaryResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMRemoteDictionaryResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource1 {
    type Vtable = IXpsOMRemoteDictionaryResource1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResourceCollection(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMRemoteDictionaryResource>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMRemoteDictionaryResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMRemoteDictionaryResourceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResourceCollection {
    type Vtable = IXpsOMRemoteDictionaryResourceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c38db61_7fec_464a_87bd_41e3bef018be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMResource(::windows::core::IUnknown);
impl IXpsOMResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMResource {
    type Vtable = IXpsOMResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMShareable(::windows::core::IUnknown);
impl IXpsOMShareable {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMShareable {
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
unsafe impl ::windows::core::Interface for IXpsOMShareable {
    type Vtable = IXpsOMShareableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResource(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMDocument>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMSignatureBlockResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResource {
    type Vtable = IXpsOMSignatureBlockResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4776ad35_2e04_4357_8743_ebf6c171a905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResourceCollection(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, index: u32, signatureblockresource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), signatureblockresource.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMSignatureBlockResource>>(&self, signatureblockresource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), signatureblockresource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetByPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSignatureBlockResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMSignatureBlockResourceCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResourceCollection {
    type Vtable = IXpsOMSignatureBlockResourceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMSolidColorBrush(::windows::core::IUnknown);
impl IXpsOMSolidColorBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMColorProfileResource>>(&self, color: *const XPS_COLOR, colorprofile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(color), colorprofile.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMSolidColorBrush>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMSolidColorBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMSolidColorBrush {
    type Vtable = IXpsOMSolidColorBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06f9f05_3be9_4763_98a8_094fc672e488);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMStoryFragmentsResource(::windows::core::IUnknown);
impl IXpsOMStoryFragmentsResource {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMPageReference>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, sourcestream: Param0, partname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMResource> for &IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMResource> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMPart> for &IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMPart> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMStoryFragmentsResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMStoryFragmentsResource {
    type Vtable = IXpsOMStoryFragmentsResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b3ca09_0473_4282_87ae_1780863223f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMStoryFragmentsResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IXpsOMThumbnailGenerator(::windows::core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GenerateThumbnail<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMPage>, Param3: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, page: Param0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: Param3) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), page.into_param().abi(), ::core::mem::transmute(thumbnailtype), ::core::mem::transmute(thumbnailsize), imageresourcepartname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMImageResource>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMThumbnailGenerator {
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
unsafe impl ::windows::core::Interface for IXpsOMThumbnailGenerator {
    type Vtable = IXpsOMThumbnailGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15b873d5_1971_41e8_83a3_6578403064c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMTileBrush(::windows::core::IUnknown);
impl IXpsOMTileBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMTileBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMTileBrush {
    type Vtable = IXpsOMTileBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fc2328d_d722_4a54_b2ec_be90218a789e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMTileBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMVisual(::windows::core::IUnknown);
impl IXpsOMVisual {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, matrixtransform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), matrixtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMGeometry>(result__)
    }
    pub unsafe fn SetClipGeometryLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMGeometry>>(&self, clipgeometry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clipgeometry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClipGeometryLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMBrush>(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMBrush>>(&self, opacitymaskbrush: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), opacitymaskbrush.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOpacityMaskBrushLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ishyperlink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ishyperlink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IUri>>(&self, hyperlinkuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hyperlinkuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, language: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), language.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMVisual {
    type Vtable = IXpsOMVisualVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IXpsOMVisualBrush(::windows::core::IUnknown);
impl IXpsOMVisualBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__: XPS_OBJECT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_OBJECT_TYPE>(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMMatrixTransform>(result__)
    }
    pub unsafe fn SetTransformLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMMatrixTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransformLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewbox)).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__: XPS_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_RECT>(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__: XPS_TILE_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_TILE_MODE>(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(tilemode)).ok()
    }
    pub unsafe fn GetVisual(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    pub unsafe fn GetVisualLocal(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    pub unsafe fn SetVisualLocal<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVisualLookup(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisualLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lookup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisualBrush>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMTileBrush> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMTileBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMBrush> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMBrush> {
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
impl<'a> ::windows::core::IntoParam<'a, IXpsOMShareable> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, IXpsOMShareable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMVisualBrush {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IXpsOMVisualBrush {
    type Vtable = IXpsOMVisualBrushVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97e294af_5b37_46b4_8057_874d2f64119b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrushVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsOMVisualCollection(::windows::core::IUnknown);
impl IXpsOMVisualCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsOMVisual>(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, index: u32, object: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), object.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, IXpsOMVisual>>(&self, object: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsOMVisualCollection {
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
unsafe impl ::windows::core::Interface for IXpsOMVisualCollection {
    type Vtable = IXpsOMVisualCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSignature(::windows::core::IUnknown);
impl IXpsSignature {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturehashvalue), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcCertificateEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSigningTime(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<XPS_SIGNATURE_STATUS> {
        let mut result__: XPS_SIGNATURE_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(x509certificate), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGNATURE_STATUS>(result__)
    }
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__: XPS_SIGN_POLICY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SetSignatureXml(&self, signaturexml: *const u8, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignature {
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
unsafe impl ::windows::core::Interface for IXpsSignature {
    type Vtable = IXpsSignatureVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ae4c93e_1ade_42fb_898b_3a5658284857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSignatureBlock(::windows::core::IUnknown);
impl IXpsSignatureBlock {
    pub unsafe fn GetRequests(&self) -> ::windows::core::Result<IXpsSignatureRequestCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequestCollection>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    pub unsafe fn GetDocumentIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetDocumentName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, requestid: Param0) -> ::windows::core::Result<IXpsSignatureRequest> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequest>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureBlock {
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
unsafe impl ::windows::core::Interface for IXpsSignatureBlock {
    type Vtable = IXpsSignatureBlockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151fac09_0b97_4ac6_a323_5e4297d4322b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IXpsSignatureBlockCollection(::windows::core::IUnknown);
impl IXpsSignatureBlockCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureBlock> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlock>(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureBlockCollection {
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
unsafe impl ::windows::core::Interface for IXpsSignatureBlockCollection {
    type Vtable = IXpsSignatureBlockCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23397050_fe99_467a_8dce_9237f074ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSignatureCollection(::windows::core::IUnknown);
impl IXpsSignatureCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureCollection {
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
unsafe impl ::windows::core::Interface for IXpsSignatureCollection {
    type Vtable = IXpsSignatureCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSignatureManager(::windows::core::IUnknown);
impl IXpsSignatureManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadPackageFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<'a, Param0: ::windows::core::IntoParam<'a, IXpsSigningOptions>>(&self, signoptions: Param0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), signoptions.into_param().abi(), ::core::mem::transmute(x509certificate), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSignatureOriginPartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signatureoriginpartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), signatureoriginpartname.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatures(&self) -> ::windows::core::Result<IXpsSignatureCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureCollection>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn AddSignatureBlock<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, partname: Param0, fixeddocumentindex: u32) -> ::windows::core::Result<IXpsSignatureBlock> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), partname.into_param().abi(), ::core::mem::transmute(fixeddocumentindex), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlock>(result__)
    }
    pub unsafe fn GetSignatureBlocks(&self) -> ::windows::core::Result<IXpsSignatureBlockCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureBlockCollection>(result__)
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::core::Result<IXpsSigningOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSigningOptions>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn SavePackageToFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(flagsandattributes)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureManager {
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
unsafe impl ::windows::core::Interface for IXpsSignatureManager {
    type Vtable = IXpsSignatureManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IXpsSignatureRequest(::windows::core::IUnknown);
impl IXpsSignatureRequest {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIntent(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIntent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, intent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), intent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRequestedSigner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRequestedSigner<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRequestSignByDate(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRequestSignByDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, datestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), datestring.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSigningLocale(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSigningLocale<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, place: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), place.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(pagepartname), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pageindex), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRequestId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<IXpsSignature> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignature>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureRequest {
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
unsafe impl ::windows::core::Interface for IXpsSignatureRequest {
    type Vtable = IXpsSignatureRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSignatureRequestCollection(::windows::core::IUnknown);
impl IXpsSignatureRequestCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureRequest> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IXpsSignatureRequest>(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSignatureRequestCollection {
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
unsafe impl ::windows::core::Interface for IXpsSignatureRequestCollection {
    type Vtable = IXpsSignatureRequestCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IXpsSigningOptions(::windows::core::IUnknown);
impl IXpsSigningOptions {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSignatureId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signatureid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), signatureid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSignatureMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, signaturemethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signaturemethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDigestMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, digestmethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), digestmethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSignaturePartName<'a, Param0: ::windows::core::IntoParam<'a, super::Packaging::Opc::IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__: XPS_SIGN_POLICY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_POLICY>(result__)
    }
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeformat)).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureCustomObjectSet>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcSignatureReferenceSet>(result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Packaging::Opc::IOpcCertificateSet>(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<XPS_SIGN_FLAGS> {
        let mut result__: XPS_SIGN_FLAGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XPS_SIGN_FLAGS>(result__)
    }
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXpsSigningOptions {
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
unsafe impl ::windows::core::Interface for IXpsSigningOptions {
    type Vtable = IXpsSigningOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7718eae4_3215_49be_af5b_594fef7fcfa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSigningOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
);
pub type PRINT_WINDOW_FLAGS = u32;
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = 1u32;
#[repr(C)]
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
pub type PSINJECT_POINT = u16;
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = 1u16;
pub const PSINJECT_PSADOBE: PSINJECT_POINT = 2u16;
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = 3u16;
pub const PSINJECT_PAGES: PSINJECT_POINT = 4u16;
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = 5u16;
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = 6u16;
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = 7u16;
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = 8u16;
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = 9u16;
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = 10u16;
pub const PSINJECT_COMMENTS: PSINJECT_POINT = 11u16;
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = 12u16;
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = 13u16;
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = 14u16;
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = 15u16;
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = 16u16;
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = 17u16;
pub const PSINJECT_TRAILER: PSINJECT_POINT = 18u16;
pub const PSINJECT_EOF: PSINJECT_POINT = 19u16;
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = 20u16;
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = 21u16;
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = 100u16;
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = 101u16;
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = 102u16;
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = 103u16;
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = 104u16;
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = 105u16;
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = 106u16;
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = 107u16;
pub const PSINJECT_VMSAVE: PSINJECT_POINT = 200u16;
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = 201u16;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
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
pub type XPS_COLOR_INTERPOLATION = i32;
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = 1i32;
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = 2i32;
pub type XPS_COLOR_TYPE = i32;
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = 1i32;
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = 2i32;
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = 3i32;
#[repr(C)]
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
pub type XPS_DASH_CAP = i32;
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = 1i32;
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = 2i32;
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = 3i32;
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = 4i32;
pub type XPS_DOCUMENT_TYPE = i32;
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = 1i32;
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = 2i32;
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = 3i32;
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108159i32);
pub const XPS_E_ALREADY_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108413i32);
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108407i32);
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108409i32);
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108408i32);
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108923i32);
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108922i32);
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108410i32);
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108671i32);
pub const XPS_E_DUPLICATE_NAMES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109175i32);
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109184i32);
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108416i32);
pub const XPS_E_INVALID_BLEED_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109692i32);
pub const XPS_E_INVALID_CONTENT_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109685i32);
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109682i32);
pub const XPS_E_INVALID_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109689i32);
pub const XPS_E_INVALID_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109686i32);
pub const XPS_E_INVALID_LANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109696i32);
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109690i32);
pub const XPS_E_INVALID_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109684i32);
pub const XPS_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109695i32);
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108158i32);
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108160i32);
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109681i32);
pub const XPS_E_INVALID_PAGE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109693i32);
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109694i32);
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108789i32);
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109691i32);
pub const XPS_E_INVALID_XML_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109683i32);
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108924i32);
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108925i32);
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108926i32);
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108791i32);
pub const XPS_E_MISSING_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109436i32);
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109422i32);
pub const XPS_E_MISSING_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109431i32);
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109432i32);
pub const XPS_E_MISSING_FONTURI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109433i32);
pub const XPS_E_MISSING_GLYPHS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109438i32);
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109426i32);
pub const XPS_E_MISSING_LOOKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109439i32);
pub const XPS_E_MISSING_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109440i32);
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109428i32);
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109427i32);
pub const XPS_E_MISSING_PART_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109424i32);
pub const XPS_E_MISSING_PART_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109421i32);
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109430i32);
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109429i32);
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109435i32);
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109425i32);
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109434i32);
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109423i32);
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109437i32);
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109182i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109178i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109177i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109179i32);
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109176i32);
pub const XPS_E_MULTIPLE_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109183i32);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109180i32);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109181i32);
pub const XPS_E_NEGATIVE_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108918i32);
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108670i32);
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108405i32);
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108414i32);
pub const XPS_E_OBJECT_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108790i32);
pub const XPS_E_ODD_BIDILEVEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108921i32);
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108920i32);
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108793i32);
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108794i32);
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108404i32);
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108406i32);
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108412i32);
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108919i32);
pub const XPS_E_SIGNATUREID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108792i32);
pub const XPS_E_SIGREQUESTID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108795i32);
pub const XPS_E_STRING_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108928i32);
pub const XPS_E_TOO_MANY_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108927i32);
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109420i32);
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108411i32);
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109688i32);
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109680i32);
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109679i32);
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108415i32);
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108672i32);
pub type XPS_FILL_RULE = i32;
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = 1i32;
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = 2i32;
pub type XPS_FONT_EMBEDDING = i32;
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = 1i32;
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = 2i32;
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = 3i32;
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = 4i32;
#[repr(C)]
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
pub type XPS_IMAGE_TYPE = i32;
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = 1i32;
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = 2i32;
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = 3i32;
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = 4i32;
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = 5i32;
pub type XPS_INTERLEAVING = i32;
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = 1i32;
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = 2i32;
pub type XPS_LINE_CAP = i32;
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = 1i32;
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = 2i32;
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = 3i32;
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = 4i32;
pub type XPS_LINE_JOIN = i32;
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = 1i32;
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = 2i32;
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = 3i32;
#[repr(C)]
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
pub type XPS_OBJECT_TYPE = i32;
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = 1i32;
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = 2i32;
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = 3i32;
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = 4i32;
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = 5i32;
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = 6i32;
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = 7i32;
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 8i32;
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 9i32;
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = 10i32;
#[repr(C)]
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
pub type XPS_SEGMENT_STROKE_PATTERN = i32;
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = 1i32;
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = 2i32;
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = 3i32;
pub type XPS_SEGMENT_TYPE = i32;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = 1i32;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 2i32;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = 3i32;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 4i32;
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = 5i32;
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = 6i32;
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = 7i32;
pub type XPS_SIGNATURE_STATUS = i32;
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = 1i32;
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = 2i32;
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = 3i32;
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = 4i32;
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = 5i32;
pub type XPS_SIGN_FLAGS = i32;
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = 0i32;
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = 1i32;
pub type XPS_SIGN_POLICY = i32;
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = 0i32;
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = 1i32;
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = 2i32;
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = 4i32;
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = 8i32;
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = 15i32;
#[repr(C)]
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
pub type XPS_SPREAD_METHOD = i32;
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = 1i32;
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = 2i32;
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = 3i32;
pub type XPS_STYLE_SIMULATION = i32;
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = 1i32;
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = 2i32;
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = 3i32;
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = 4i32;
pub type XPS_THUMBNAIL_SIZE = i32;
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = 1i32;
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = 2i32;
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = 3i32;
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = 4i32;
pub type XPS_TILE_MODE = i32;
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = 1i32;
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = 2i32;
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = 3i32;
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = 4i32;
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = 5i32;
pub const XpsOMObjectFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);

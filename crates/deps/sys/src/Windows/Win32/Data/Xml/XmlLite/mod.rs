#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlReader(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingName(pinputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, pwszencodingname: super::super::super::Foundation::PWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriter(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, nencodingcodepage: u32, ppoutput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlWriterOutputWithEncodingName(poutputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, pwszencodingname: super::super::super::Foundation::PWSTR, ppoutput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
pub struct DtdProcessing(i32);
pub struct IXmlReader(pub *mut ::core::ffi::c_void);
pub struct IXmlResolver(pub *mut ::core::ffi::c_void);
pub struct IXmlWriter(pub *mut ::core::ffi::c_void);
pub struct IXmlWriterLite(pub *mut ::core::ffi::c_void);
pub struct XmlConformanceLevel(i32);
pub struct XmlError(i32);
pub struct XmlNodeType(i32);
pub struct XmlReadState(i32);
pub struct XmlReaderProperty(i32);
pub struct XmlStandalone(i32);
pub struct XmlWriterProperty(i32);
pub const _IID_IXmlReader: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597121, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlResolver: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597122, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlWriter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597128, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_XMLGraphBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`*"]
#[repr(transparent)]
pub struct IXMLGraphBuilder(::windows::core::IUnknown);
impl IXMLGraphBuilder {
    #[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn BuildFromXML<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::core::IntoParam<'a, super::super::super::Data::Xml::MsXml::IXMLElement>>(&self, pgraph: Param0, pxml: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BuildFromXML)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), pxml.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveToXML<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>>(&self, pgraph: Param0, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveToXML)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`*"]
    pub unsafe fn BuildFromXMLFile<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pgraph: Param0, wszfilename: Param1, wszbaseurl: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BuildFromXMLFile)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), wszfilename.into_param().abi(), wszbaseurl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: IXMLGraphBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: &IXMLGraphBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXMLGraphBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXMLGraphBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXMLGraphBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXMLGraphBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLGraphBuilder {}
impl ::core::fmt::Debug for IXMLGraphBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLGraphBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLGraphBuilder {
    type Vtable = IXMLGraphBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb05960_5fbf_11d2_a521_44df07c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pxml: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveToXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveToXML: usize,
    pub BuildFromXMLFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, wszfilename: ::windows::core::PCWSTR, wszbaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

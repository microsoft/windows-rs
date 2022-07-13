pub const CLSID_XMLGraphBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`*"]
#[repr(transparent)]
pub struct IXMLGraphBuilder(::windows::core::IUnknown);
impl IXMLGraphBuilder {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn BuildFromXML<'a, P0, P1>(&self, pgraph: P0, pxml: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IGraphBuilder>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Data::Xml::MsXml::IXMLElement>>,
    {
        (::windows::core::Interface::vtable(self).BuildFromXML)(::windows::core::Interface::as_raw(self), pgraph.into().abi(), pxml.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveToXML<'a, P0>(&self, pgraph: P0, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IGraphBuilder>>,
    {
        (::windows::core::Interface::vtable(self).SaveToXML)(::windows::core::Interface::as_raw(self), pgraph.into().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    pub unsafe fn BuildFromXMLFile<'a, P0, P1, P2>(&self, pgraph: P0, wszfilename: P1, wszbaseurl: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IGraphBuilder>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).BuildFromXMLFile)(::windows::core::Interface::as_raw(self), pgraph.into().abi(), wszfilename.into(), wszbaseurl.into()).ok()
    }
}
impl ::core::convert::From<IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: IXMLGraphBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXMLGraphBuilder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXMLGraphBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: &IXMLGraphBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pxml: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveToXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveToXML: usize,
    pub BuildFromXMLFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, wszfilename: ::windows::core::PCWSTR, wszbaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

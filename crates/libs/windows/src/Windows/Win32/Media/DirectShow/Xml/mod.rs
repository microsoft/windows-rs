#[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`*"]
#[repr(transparent)]
pub struct IXMLGraphBuilder(::windows_core::IUnknown);
impl IXMLGraphBuilder {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn BuildFromXML<P0, P1>(&self, pgraph: P0, pxml: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IGraphBuilder>,
        P1: ::windows_core::IntoParam<super::super::super::Data::Xml::MsXml::IXMLElement>,
    {
        (::windows_core::Interface::vtable(self).BuildFromXML)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), pxml.into_param().abi()).ok()
    }
    pub unsafe fn SaveToXML<P0>(&self, pgraph: P0, pbstrxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IGraphBuilder>,
    {
        (::windows_core::Interface::vtable(self).SaveToXML)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    pub unsafe fn BuildFromXMLFile<P0, P1, P2>(&self, pgraph: P0, wszfilename: P1, wszbaseurl: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IGraphBuilder>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).BuildFromXMLFile)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), wszfilename.into_param().abi(), wszbaseurl.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXMLGraphBuilder, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IXMLGraphBuilder {
    type Vtable = IXMLGraphBuilder_Vtbl;
}
impl ::core::clone::Clone for IXMLGraphBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXMLGraphBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb05960_5fbf_11d2_a521_44df07c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pxml: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    pub SaveToXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BuildFromXMLFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, wszfilename: ::windows_core::PCWSTR, wszbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`*"]
pub const CLSID_XMLGraphBuilder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_XMLGraphBuilder: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[doc = "*Required features: `Win32_Media_DirectShow_Xml`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXMLGraphBuilder(pub ::windows::runtime::IUnknown);
impl IXMLGraphBuilder {
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    #[doc = "*Required features: `Win32_Media_DirectShow_Xml`, `Win32_Data_Xml_MsXml`*"]
    pub unsafe fn BuildFromXML<'a, Param0: ::windows::runtime::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Data::Xml::MsXml::IXMLElement>>(&self, pgraph: Param0, pxml: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), pxml.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DirectShow_Xml`, `Win32_Foundation`*"]
    pub unsafe fn SaveToXML<'a, Param0: ::windows::runtime::IntoParam<'a, super::IGraphBuilder>>(&self, pgraph: Param0, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DirectShow_Xml`, `Win32_Foundation`*"]
    pub unsafe fn BuildFromXMLFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pgraph: Param0, wszfilename: Param1, wszbaseurl: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), wszfilename.into_param().abi(), wszbaseurl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXMLGraphBuilder {
    type Vtable = IXMLGraphBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1bb05960_5fbf_11d2_a521_44df07c10000);
}
impl ::core::convert::From<IXMLGraphBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IXMLGraphBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXMLGraphBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IXMLGraphBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXMLGraphBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXMLGraphBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Data_Xml_MsXml")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgraph: ::windows::runtime::RawPtr, pxml: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgraph: ::windows::runtime::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgraph: ::windows::runtime::RawPtr, wszfilename: super::super::super::Foundation::PWSTR, wszbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);

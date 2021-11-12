#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_XMLGraphBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXMLGraphBuilder(pub ::windows::core::IUnknown);
impl IXMLGraphBuilder {
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn BuildFromXML<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::core::IntoParam<'a, super::super::super::Data::Xml::MsXml::IXMLElement>>(&self, pgraph: Param0, pxml: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), pxml.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveToXML<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>>(&self, pgraph: Param0, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildFromXMLFile<'a, Param0: ::windows::core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pgraph: Param0, wszfilename: Param1, wszbaseurl: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pgraph.into_param().abi(), wszfilename.into_param().abi(), wszbaseurl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IXMLGraphBuilder {
    type Vtable = IXMLGraphBuilder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb05960_5fbf_11d2_a521_44df07c10000);
}
impl ::core::convert::From<IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: IXMLGraphBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXMLGraphBuilder> for ::windows::core::IUnknown {
    fn from(value: &IXMLGraphBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXMLGraphBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXMLGraphBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Data_Xml_MsXml")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgraph: ::windows::core::RawPtr, pxml: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgraph: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgraph: ::windows::core::RawPtr, wszfilename: super::super::super::Foundation::PWSTR, wszbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);

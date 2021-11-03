#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DtdEntity(::windows::runtime::IInspectable);
impl DtdEntity {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PublicId(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SystemId(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NotationName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
}
unsafe impl ::windows::runtime::Interface for DtdEntity {
    type Vtable = IDtdEntity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1779130364, 25524, 18447, [158, 106, 138, 146, 129, 106, 173, 228]);
}
impl ::windows::runtime::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
impl ::std::convert::From<DtdEntity> for ::windows::runtime::IUnknown {
    fn from(value: DtdEntity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DtdEntity> for ::windows::runtime::IUnknown {
    fn from(value: &DtdEntity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DtdEntity> for ::windows::runtime::IInspectable {
    fn from(value: DtdEntity) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DtdEntity> for ::windows::runtime::IInspectable {
    fn from(value: &DtdEntity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DtdEntity> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdEntity) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdEntity> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdEntity) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DtdEntity> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdEntity) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdEntity> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdEntity) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdEntity) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdEntity) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &DtdEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DtdEntity {}
unsafe impl ::std::marker::Sync for DtdEntity {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DtdNotation(::windows::runtime::IInspectable);
impl DtdNotation {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PublicId(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SystemId(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
}
unsafe impl ::windows::runtime::Interface for DtdNotation {
    type Vtable = IDtdNotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360664141, 27974, 20187, [171, 115, 223, 131, 197, 26, 211, 151]);
}
impl ::windows::runtime::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
impl ::std::convert::From<DtdNotation> for ::windows::runtime::IUnknown {
    fn from(value: DtdNotation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DtdNotation> for ::windows::runtime::IUnknown {
    fn from(value: &DtdNotation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DtdNotation> for ::windows::runtime::IInspectable {
    fn from(value: DtdNotation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DtdNotation> for ::windows::runtime::IInspectable {
    fn from(value: &DtdNotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DtdNotation> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdNotation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdNotation> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdNotation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DtdNotation> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdNotation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdNotation> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdNotation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DtdNotation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DtdNotation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &DtdNotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DtdNotation {}
unsafe impl ::std::marker::Sync for DtdNotation {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDtdEntity(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDtdEntity {
    type Vtable = IDtdEntity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1779130364, 25524, 18447, [158, 106, 138, 146, 129, 106, 173, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDtdNotation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDtdNotation {
    type Vtable = IDtdNotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360664141, 27974, 20187, [171, 115, 223, 131, 197, 26, 211, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlAttribute(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2887010980, 46321, 19894, [178, 6, 138, 34, 195, 8, 219, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlCDataSection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1292153967, 51389, 17844, [136, 153, 4, 0, 215, 194, 198, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlCharacterData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(321798827, 20022, 19958, [177, 200, 12, 230, 47, 216, 139, 38]);
}
impl IXmlCharacterData {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
}
impl ::std::convert::From<IXmlCharacterData> for ::windows::runtime::IUnknown {
    fn from(value: IXmlCharacterData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlCharacterData> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlCharacterData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IXmlCharacterData> for ::windows::runtime::IInspectable {
    fn from(value: IXmlCharacterData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXmlCharacterData> for ::windows::runtime::IInspectable {
    fn from(value: &IXmlCharacterData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: u32, count: u32, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: u32, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: u32, count: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: u32, count: u32, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlComment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlComment {
    type Vtable = IXmlComment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3164894421, 46623, 17937, [156, 172, 46, 146, 227, 71, 109, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocument(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocument {
    type Vtable = IXmlDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4159939846, 7815, 17110, [188, 251, 184, 200, 9, 250, 84, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tagname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tagname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, qualifiedname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, qualifiedname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, elementid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, node: ::windows::runtime::RawPtr, deep: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentFragment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3807013526, 3105, 17573, [139, 201, 158, 74, 38, 39, 8, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentIO(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1825630030, 61029, 17545, [158, 191, 202, 67, 232, 123, 166, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, loadsettings: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentIO2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1560495713, 31704, 19157, [158, 191, 129, 230, 52, 114, 99, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, loadsettings: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1430508116, 55127, 19321, [149, 57, 35, 43, 24, 245, 11, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, loadsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, loadsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentType(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4147389477, 38785, 18788, [142, 148, 155, 28, 109, 252, 155, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDomImplementation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1843757362, 61725, 20411, [140, 198, 88, 60, 186, 147, 17, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, feature: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, version: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlElement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlElement {
    type Vtable = IXmlElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(771459615, 27408, 20216, [159, 131, 239, 204, 232, 250, 236, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, attributevalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newattribute: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributenode: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tagname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, qualifiedname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, localname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, localname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newattribute: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, localname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlEntityReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(774850492, 50128, 19663, [187, 134, 10, 184, 195, 106, 97, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlLoadSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1487538088, 65238, 18167, [180, 197, 251, 27, 167, 33, 8, 214]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3014041264, 43696, 19330, [166, 250, 177, 69, 63, 124, 2, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, node: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceuri: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, node: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlNode {
    type Vtable = IXmlNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(477371737, 8482, 18389, [168, 86, 131, 243, 212, 33, 72, 117]);
}
impl IXmlNode {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
}
impl ::std::convert::From<IXmlNode> for ::windows::runtime::IUnknown {
    fn from(value: IXmlNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlNode> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IXmlNode> for ::windows::runtime::IInspectable {
    fn from(value: IXmlNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXmlNode> for ::windows::runtime::IInspectable {
    fn from(value: &IXmlNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IXmlNode> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlNode> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &IXmlNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NodeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newchild: ::windows::runtime::RawPtr, referencechild: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newchild: ::windows::runtime::RawPtr, referencechild: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, childnode: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newchild: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deep: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlNodeList(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlNodeList {
    type Vtable = IXmlNodeList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2355146103, 33700, 20161, [156, 84, 123, 164, 41, 225, 61, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNodeSelector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1675344523, 53467, 20449, [183, 69, 249, 67, 58, 253, 194, 91]);
}
impl IXmlNodeSelector {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
}
impl ::std::convert::From<IXmlNodeSelector> for ::windows::runtime::IUnknown {
    fn from(value: IXmlNodeSelector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlNodeSelector> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlNodeSelector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlNodeSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlNodeSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IXmlNodeSelector> for ::windows::runtime::IInspectable {
    fn from(value: IXmlNodeSelector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXmlNodeSelector> for ::windows::runtime::IInspectable {
    fn from(value: &IXmlNodeSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXmlNodeSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXmlNodeSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, namespaces: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, namespaces: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNodeSerializer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1556460418, 59101, 18833, [171, 239, 6, 216, 210, 231, 189, 12]);
}
impl IXmlNodeSerializer {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
}
impl ::std::convert::From<IXmlNodeSerializer> for ::windows::runtime::IUnknown {
    fn from(value: IXmlNodeSerializer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlNodeSerializer> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlNodeSerializer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlNodeSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IXmlNodeSerializer> for ::windows::runtime::IInspectable {
    fn from(value: IXmlNodeSerializer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXmlNodeSerializer> for ::windows::runtime::IInspectable {
    fn from(value: &IXmlNodeSerializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXmlNodeSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(654834974, 7826, 20174, [182, 244, 38, 240, 105, 7, 141, 220]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlText(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXmlText {
    type Vtable = IXmlText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4180780235, 12429, 18272, [161, 213, 67, 182, 116, 80, 172, 126]);
}
impl IXmlText {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::runtime::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXmlText {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
}
impl ::std::convert::From<IXmlText> for ::windows::runtime::IUnknown {
    fn from(value: IXmlText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlText> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IXmlText> for ::windows::runtime::IInspectable {
    fn from(value: IXmlText) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXmlText> for ::windows::runtime::IInspectable {
    fn from(value: &IXmlText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IXmlText> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlText> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for &IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlText> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlText> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlText> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlText> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IXmlText> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXmlText> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &IXmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NodeType(pub i32);
impl NodeType {
    pub const Invalid: NodeType = NodeType(0i32);
    pub const ElementNode: NodeType = NodeType(1i32);
    pub const AttributeNode: NodeType = NodeType(2i32);
    pub const TextNode: NodeType = NodeType(3i32);
    pub const DataSectionNode: NodeType = NodeType(4i32);
    pub const EntityReferenceNode: NodeType = NodeType(5i32);
    pub const EntityNode: NodeType = NodeType(6i32);
    pub const ProcessingInstructionNode: NodeType = NodeType(7i32);
    pub const CommentNode: NodeType = NodeType(8i32);
    pub const DocumentNode: NodeType = NodeType(9i32);
    pub const DocumentTypeNode: NodeType = NodeType(10i32);
    pub const DocumentFragmentNode: NodeType = NodeType(11i32);
    pub const NotationNode: NodeType = NodeType(12i32);
}
impl ::std::convert::From<i32> for NodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NodeType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NodeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
}
impl ::windows::runtime::DefaultType for NodeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlAttribute(::windows::runtime::IInspectable);
impl XmlAttribute {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Specified(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
}
unsafe impl ::windows::runtime::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2887010980, 46321, 19894, [178, 6, 138, 34, 195, 8, 219, 10]);
}
impl ::windows::runtime::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
impl ::std::convert::From<XmlAttribute> for ::windows::runtime::IUnknown {
    fn from(value: XmlAttribute) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlAttribute> for ::windows::runtime::IUnknown {
    fn from(value: &XmlAttribute) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlAttribute> for ::windows::runtime::IInspectable {
    fn from(value: XmlAttribute) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlAttribute> for ::windows::runtime::IInspectable {
    fn from(value: &XmlAttribute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlAttribute> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlAttribute {}
unsafe impl ::std::marker::Sync for XmlAttribute {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlCDataSection(::windows::runtime::IInspectable);
impl XmlCDataSection {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::runtime::Result<IXmlText> {
        let this = &::windows::runtime::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
}
unsafe impl ::windows::runtime::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1292153967, 51389, 17844, [136, 153, 4, 0, 215, 194, 198, 15]);
}
impl ::windows::runtime::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
impl ::std::convert::From<XmlCDataSection> for ::windows::runtime::IUnknown {
    fn from(value: XmlCDataSection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlCDataSection> for ::windows::runtime::IUnknown {
    fn from(value: &XmlCDataSection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlCDataSection> for ::windows::runtime::IInspectable {
    fn from(value: XmlCDataSection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlCDataSection> for ::windows::runtime::IInspectable {
    fn from(value: &XmlCDataSection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlCDataSection> for IXmlText {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlText {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlText> for XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlText> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlText> for &XmlCDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlText> {
        ::std::convert::TryInto::<IXmlText>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlCDataSection {}
unsafe impl ::std::marker::Sync for XmlCDataSection {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlComment(::windows::runtime::IInspectable);
impl XmlComment {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlComment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
}
unsafe impl ::windows::runtime::Interface for XmlComment {
    type Vtable = IXmlComment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3164894421, 46623, 17937, [156, 172, 46, 146, 227, 71, 109, 71]);
}
impl ::windows::runtime::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
impl ::std::convert::From<XmlComment> for ::windows::runtime::IUnknown {
    fn from(value: XmlComment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlComment> for ::windows::runtime::IUnknown {
    fn from(value: &XmlComment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlComment> for ::windows::runtime::IInspectable {
    fn from(value: XmlComment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlComment> for ::windows::runtime::IInspectable {
    fn from(value: &XmlComment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlComment> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlComment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlComment> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlComment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for &XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlComment> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlComment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlComment> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlComment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlComment> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlComment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlComment> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlComment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlComment> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlComment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlComment> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlComment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlComment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlComment {}
unsafe impl ::std::marker::Sync for XmlComment {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlDocument(::windows::runtime::IInspectable);
impl XmlDocument {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XmlDocument, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Doctype(&self) -> ::windows::runtime::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocumentType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Implementation(&self) -> ::windows::runtime::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDomImplementation>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DocumentElement(&self) -> ::windows::runtime::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateElement<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tagname: Param0) -> ::windows::runtime::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateDocumentFragment(&self) -> ::windows::runtime::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocumentFragment>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateTextNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlComment>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateProcessingInstruction<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, target: Param0, data: Param1) -> ::windows::runtime::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), target.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<XmlProcessingInstruction>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateEntityReference<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<XmlEntityReference>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementsByTagName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tagname: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateCDataSection<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlCDataSection>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DocumentUri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateAttributeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateElementNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows::runtime::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, elementid: Param0) -> ::windows::runtime::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), elementid.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ImportNode<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, node: Param0, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), node.into_param().abi(), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LoadXml<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xml: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xml.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LoadXmlWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, XmlLoadSettings>>(&self, xml: Param0, loadsettings: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xml.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Storage_Streams`*"]
    pub fn LoadXmlFromBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Storage_Streams`*"]
    pub fn LoadXmlFromBufferWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, XmlLoadSettings>>(&self, buffer: Param0, loadsettings: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), buffer.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`*"]
    pub fn LoadFromUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`*"]
    pub fn LoadFromUriWithSettingsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, XmlLoadSettings>>(uri: Param0, loadsettings: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), loadsettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn LoadFromFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn LoadFromFileWithSettingsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, XmlLoadSettings>>(file: Param0, loadsettings: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), file.into_param().abi(), loadsettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
}
unsafe impl ::windows::runtime::Interface for XmlDocument {
    type Vtable = IXmlDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4159939846, 7815, 17110, [188, 251, 184, 200, 9, 250, 84, 148]);
}
impl ::windows::runtime::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
impl ::std::convert::From<XmlDocument> for ::windows::runtime::IUnknown {
    fn from(value: XmlDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlDocument> for ::windows::runtime::IUnknown {
    fn from(value: &XmlDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlDocument> for ::windows::runtime::IInspectable {
    fn from(value: XmlDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlDocument> for ::windows::runtime::IInspectable {
    fn from(value: &XmlDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlDocument> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocument) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocument> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocument) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocument> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocument) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocument> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocument) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocument) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocument) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlDocument {}
unsafe impl ::std::marker::Sync for XmlDocument {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlDocumentFragment(::windows::runtime::IInspectable);
impl XmlDocumentFragment {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
}
unsafe impl ::windows::runtime::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3807013526, 3105, 17573, [139, 201, 158, 74, 38, 39, 8, 236]);
}
impl ::windows::runtime::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
impl ::std::convert::From<XmlDocumentFragment> for ::windows::runtime::IUnknown {
    fn from(value: XmlDocumentFragment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlDocumentFragment> for ::windows::runtime::IUnknown {
    fn from(value: &XmlDocumentFragment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlDocumentFragment> for ::windows::runtime::IInspectable {
    fn from(value: XmlDocumentFragment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlDocumentFragment> for ::windows::runtime::IInspectable {
    fn from(value: &XmlDocumentFragment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlDocumentFragment {}
unsafe impl ::std::marker::Sync for XmlDocumentFragment {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlDocumentType(::windows::runtime::IInspectable);
impl XmlDocumentType {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Entities(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Notations(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
}
unsafe impl ::windows::runtime::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4147389477, 38785, 18788, [142, 148, 155, 28, 109, 252, 155, 199]);
}
impl ::windows::runtime::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
impl ::std::convert::From<XmlDocumentType> for ::windows::runtime::IUnknown {
    fn from(value: XmlDocumentType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlDocumentType> for ::windows::runtime::IUnknown {
    fn from(value: &XmlDocumentType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlDocumentType> for ::windows::runtime::IInspectable {
    fn from(value: XmlDocumentType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlDocumentType> for ::windows::runtime::IInspectable {
    fn from(value: &XmlDocumentType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlDocumentType {}
unsafe impl ::std::marker::Sync for XmlDocumentType {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlDomImplementation(::windows::runtime::IInspectable);
impl XmlDomImplementation {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasFeature<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, feature: Param0, version: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), feature.into_param().abi(), version.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
}
unsafe impl ::windows::runtime::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1843757362, 61725, 20411, [140, 198, 88, 60, 186, 147, 17, 47]);
}
impl ::windows::runtime::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
impl ::std::convert::From<XmlDomImplementation> for ::windows::runtime::IUnknown {
    fn from(value: XmlDomImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlDomImplementation> for ::windows::runtime::IUnknown {
    fn from(value: &XmlDomImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlDomImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlDomImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlDomImplementation> for ::windows::runtime::IInspectable {
    fn from(value: XmlDomImplementation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlDomImplementation> for ::windows::runtime::IInspectable {
    fn from(value: &XmlDomImplementation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlDomImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlDomImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XmlDomImplementation {}
unsafe impl ::std::marker::Sync for XmlDomImplementation {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlElement(::windows::runtime::IInspectable);
impl XmlElement {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn TagName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, attributename: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), attributename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, attributename: Param0, attributevalue: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), attributename.into_param().abi(), attributevalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, attributename: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), attributename.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, attributename: Param0) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), attributename.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNode<'a, Param0: ::windows::runtime::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), newattribute.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttributeNode<'a, Param0: ::windows::runtime::IntoParam<'a, XmlAttribute>>(&self, attributenode: Param0) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), attributenode.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementsByTagName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tagname: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttributeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), newattribute.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::runtime::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
}
unsafe impl ::windows::runtime::Interface for XmlElement {
    type Vtable = IXmlElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(771459615, 27408, 20216, [159, 131, 239, 204, 232, 250, 236, 55]);
}
impl ::windows::runtime::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
impl ::std::convert::From<XmlElement> for ::windows::runtime::IUnknown {
    fn from(value: XmlElement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlElement> for ::windows::runtime::IUnknown {
    fn from(value: &XmlElement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlElement> for ::windows::runtime::IInspectable {
    fn from(value: XmlElement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlElement> for ::windows::runtime::IInspectable {
    fn from(value: &XmlElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlElement> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlElement) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlElement> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlElement) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlElement> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlElement) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlElement> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlElement) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlElement> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlElement) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlElement> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlElement) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlElement {}
unsafe impl ::std::marker::Sync for XmlElement {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlEntityReference(::windows::runtime::IInspectable);
impl XmlEntityReference {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
}
unsafe impl ::windows::runtime::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(774850492, 50128, 19663, [187, 134, 10, 184, 195, 106, 97, 207]);
}
impl ::windows::runtime::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
impl ::std::convert::From<XmlEntityReference> for ::windows::runtime::IUnknown {
    fn from(value: XmlEntityReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlEntityReference> for ::windows::runtime::IUnknown {
    fn from(value: &XmlEntityReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlEntityReference> for ::windows::runtime::IInspectable {
    fn from(value: XmlEntityReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlEntityReference> for ::windows::runtime::IInspectable {
    fn from(value: &XmlEntityReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlEntityReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlEntityReference {}
unsafe impl ::std::marker::Sync for XmlEntityReference {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlLoadSettings(::windows::runtime::IInspectable);
impl XmlLoadSettings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XmlLoadSettings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn MaxElementDepth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ProhibitDtd(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ResolveExternals(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetResolveExternals(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ValidateOnParse(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ElementContentWhiteSpace(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
}
unsafe impl ::windows::runtime::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1487538088, 65238, 18167, [180, 197, 251, 27, 167, 33, 8, 214]);
}
impl ::windows::runtime::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
impl ::std::convert::From<XmlLoadSettings> for ::windows::runtime::IUnknown {
    fn from(value: XmlLoadSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlLoadSettings> for ::windows::runtime::IUnknown {
    fn from(value: &XmlLoadSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlLoadSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlLoadSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlLoadSettings> for ::windows::runtime::IInspectable {
    fn from(value: XmlLoadSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlLoadSettings> for ::windows::runtime::IInspectable {
    fn from(value: &XmlLoadSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlLoadSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlLoadSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XmlLoadSettings {}
unsafe impl ::std::marker::Sync for XmlLoadSettings {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlNamedNodeMap(::windows::runtime::IInspectable);
impl XmlNamedNodeMap {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Item(&self, index: u32) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetNamedItem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNamedItem<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), node.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveNamedItem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetNamedItemNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveNamedItemNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), namespaceuri.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNamedItemNS<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), node.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IXmlNode as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
}
unsafe impl ::windows::runtime::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3014041264, 43696, 19330, [166, 250, 177, 69, 63, 124, 2, 27]);
}
impl ::windows::runtime::RuntimeName for XmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
}
impl ::std::convert::From<XmlNamedNodeMap> for ::windows::runtime::IUnknown {
    fn from(value: XmlNamedNodeMap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlNamedNodeMap> for ::windows::runtime::IUnknown {
    fn from(value: &XmlNamedNodeMap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlNamedNodeMap> for ::windows::runtime::IInspectable {
    fn from(value: XmlNamedNodeMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlNamedNodeMap> for ::windows::runtime::IInspectable {
    fn from(value: &XmlNamedNodeMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<XmlNamedNodeMap> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&XmlNamedNodeMap> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<XmlNamedNodeMap> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&XmlNamedNodeMap> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlNamedNodeMap {}
unsafe impl ::std::marker::Sync for XmlNamedNodeMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlNodeList(::windows::runtime::IInspectable);
impl XmlNodeList {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Item(&self, index: u32) -> ::windows::runtime::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IXmlNode as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
}
unsafe impl ::windows::runtime::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2355146103, 33700, 20161, [156, 84, 123, 164, 41, 225, 61, 166]);
}
impl ::windows::runtime::RuntimeName for XmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
}
impl ::std::convert::From<XmlNodeList> for ::windows::runtime::IUnknown {
    fn from(value: XmlNodeList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlNodeList> for ::windows::runtime::IUnknown {
    fn from(value: &XmlNodeList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlNodeList> for ::windows::runtime::IInspectable {
    fn from(value: XmlNodeList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlNodeList> for ::windows::runtime::IInspectable {
    fn from(value: &XmlNodeList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<XmlNodeList> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlNodeList) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&XmlNodeList> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<XmlNodeList> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlNodeList) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&XmlNodeList> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlNodeList {}
unsafe impl ::std::marker::Sync for XmlNodeList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlProcessingInstruction(::windows::runtime::IInspectable);
impl XmlProcessingInstruction {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Target(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
}
unsafe impl ::windows::runtime::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(654834974, 7826, 20174, [182, 244, 38, 240, 105, 7, 141, 220]);
}
impl ::windows::runtime::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
impl ::std::convert::From<XmlProcessingInstruction> for ::windows::runtime::IUnknown {
    fn from(value: XmlProcessingInstruction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlProcessingInstruction> for ::windows::runtime::IUnknown {
    fn from(value: &XmlProcessingInstruction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlProcessingInstruction> for ::windows::runtime::IInspectable {
    fn from(value: XmlProcessingInstruction) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlProcessingInstruction> for ::windows::runtime::IInspectable {
    fn from(value: &XmlProcessingInstruction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlProcessingInstruction {}
unsafe impl ::std::marker::Sync for XmlProcessingInstruction {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XmlText(::windows::runtime::IInspectable);
impl XmlText {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::runtime::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::runtime::Result<NodeType> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<XmlNamedNodeMap> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::runtime::Result<XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>, Param1: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xpath: Param0) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<IXmlNode> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::runtime::Result<XmlNodeList> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XmlText {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
}
unsafe impl ::windows::runtime::Interface for XmlText {
    type Vtable = IXmlText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4180780235, 12429, 18272, [161, 213, 67, 182, 116, 80, 172, 126]);
}
impl ::windows::runtime::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
impl ::std::convert::From<XmlText> for ::windows::runtime::IUnknown {
    fn from(value: XmlText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlText> for ::windows::runtime::IUnknown {
    fn from(value: &XmlText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XmlText> for ::windows::runtime::IInspectable {
    fn from(value: XmlText) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XmlText> for ::windows::runtime::IInspectable {
    fn from(value: &XmlText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<XmlText> for IXmlText {
    fn from(value: XmlText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XmlText> for IXmlText {
    fn from(value: &XmlText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlText> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlText> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXmlText>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlText> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlText> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXmlText>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<XmlText> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlText> for IXmlCharacterData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlCharacterData> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlCharacterData> {
        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlText> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlText> for IXmlNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNode> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNode> {
        ::std::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlText> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlText> for IXmlNodeSelector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSelector> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSelector> {
        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<XmlText> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XmlText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XmlText> for IXmlNodeSerializer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XmlText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXmlNodeSerializer> for &XmlText {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXmlNodeSerializer> {
        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XmlText {}
unsafe impl ::std::marker::Sync for XmlText {}

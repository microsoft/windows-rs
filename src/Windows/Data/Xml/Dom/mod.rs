#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DtdEntity(pub ::windows::core::IInspectable);
impl DtdEntity {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
}
unsafe impl ::windows::core::Interface for DtdEntity {
    type Vtable = IDtdEntity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
impl ::windows::core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
impl ::core::convert::From<DtdEntity> for ::windows::core::IUnknown {
    fn from(value: DtdEntity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows::core::IUnknown {
    fn from(value: &DtdEntity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DtdEntity> for ::windows::core::IInspectable {
    fn from(value: DtdEntity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows::core::IInspectable {
    fn from(value: &DtdEntity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdEntity) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdEntity) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdEntity) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &DtdEntity {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DtdEntity {}
unsafe impl ::core::marker::Sync for DtdEntity {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DtdNotation(pub ::windows::core::IInspectable);
impl DtdNotation {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
}
unsafe impl ::windows::core::Interface for DtdNotation {
    type Vtable = IDtdNotation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
impl ::windows::core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
impl ::core::convert::From<DtdNotation> for ::windows::core::IUnknown {
    fn from(value: DtdNotation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows::core::IUnknown {
    fn from(value: &DtdNotation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DtdNotation> for ::windows::core::IInspectable {
    fn from(value: DtdNotation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows::core::IInspectable {
    fn from(value: &DtdNotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdNotation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdNotation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: DtdNotation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &DtdNotation {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDtdEntity(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDtdEntity {
    type Vtable = IDtdEntity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDtdNotation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDtdNotation {
    type Vtable = IDtdNotation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlAttribute(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlCDataSection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlCharacterData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
impl IXmlCharacterData {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
}
impl ::core::convert::From<IXmlCharacterData> for ::windows::core::IUnknown {
    fn from(value: IXmlCharacterData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows::core::IUnknown {
    fn from(value: &IXmlCharacterData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXmlCharacterData> for ::windows::core::IInspectable {
    fn from(value: IXmlCharacterData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows::core::IInspectable {
    fn from(value: &IXmlCharacterData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &IXmlCharacterData {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u32, count: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u32, count: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlComment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlComment {
    type Vtable = IXmlComment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocument(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocument {
    type Vtable = IXmlDocument_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, elementid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, node: ::windows::core::RawPtr, deep: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentFragment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentIO(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loadsettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentIO2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5543d254_d757_4b79_9539_232b18f50bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDocumentType(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlDomImplementation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, version: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlElement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlElement {
    type Vtable = IXmlElement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, attributevalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newattribute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributenode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newattribute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlEntityReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlLoadSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, node: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namespaceuri: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, node: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlNode {
    type Vtable = IXmlNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
impl IXmlNode {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
}
impl ::core::convert::From<IXmlNode> for ::windows::core::IUnknown {
    fn from(value: IXmlNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows::core::IUnknown {
    fn from(value: &IXmlNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXmlNode> for ::windows::core::IInspectable {
    fn from(value: IXmlNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows::core::IInspectable {
    fn from(value: &IXmlNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXmlNode> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlNode> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &IXmlNode {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NodeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, childnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newchild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deep: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlNodeList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlNodeList {
    type Vtable = IXmlNodeList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNodeSelector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
impl IXmlNodeSelector {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows::core::IUnknown {
    fn from(value: IXmlNodeSelector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows::core::IUnknown {
    fn from(value: &IXmlNodeSelector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlNodeSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlNodeSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows::core::IInspectable {
    fn from(value: IXmlNodeSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows::core::IInspectable {
    fn from(value: &IXmlNodeSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXmlNodeSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXmlNodeSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlNodeSerializer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
impl IXmlNodeSerializer {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows::core::IUnknown {
    fn from(value: IXmlNodeSerializer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows::core::IUnknown {
    fn from(value: &IXmlNodeSerializer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlNodeSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows::core::IInspectable {
    fn from(value: IXmlNodeSerializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows::core::IInspectable {
    fn from(value: &IXmlNodeSerializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXmlNodeSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Data_Xml_Dom`*"]
pub struct IXmlText(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXmlText {
    type Vtable = IXmlText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
impl IXmlText {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
}
impl ::core::convert::From<IXmlText> for ::windows::core::IUnknown {
    fn from(value: IXmlText) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXmlText> for ::windows::core::IUnknown {
    fn from(value: &IXmlText) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXmlText> for ::windows::core::IInspectable {
    fn from(value: IXmlText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlText> for ::windows::core::IInspectable {
    fn from(value: &IXmlText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for &IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: IXmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &IXmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for NodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NodeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NodeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
}
impl ::windows::core::DefaultType for NodeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlAttribute(pub ::windows::core::IInspectable);
impl XmlAttribute {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Specified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
}
unsafe impl ::windows::core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
impl ::windows::core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
impl ::core::convert::From<XmlAttribute> for ::windows::core::IUnknown {
    fn from(value: XmlAttribute) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows::core::IUnknown {
    fn from(value: &XmlAttribute) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlAttribute> for ::windows::core::IInspectable {
    fn from(value: XmlAttribute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows::core::IInspectable {
    fn from(value: &XmlAttribute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlAttribute {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlAttribute {}
unsafe impl ::core::marker::Sync for XmlAttribute {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlCDataSection(pub ::windows::core::IInspectable);
impl XmlCDataSection {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = &::windows::core::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
}
unsafe impl ::windows::core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
impl ::windows::core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
impl ::core::convert::From<XmlCDataSection> for ::windows::core::IUnknown {
    fn from(value: XmlCDataSection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows::core::IUnknown {
    fn from(value: &XmlCDataSection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlCDataSection> for ::windows::core::IInspectable {
    fn from(value: XmlCDataSection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows::core::IInspectable {
    fn from(value: &XmlCDataSection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for &XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlText {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlText {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlText> for XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlText> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlText> for &XmlCDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlText> {
        ::core::convert::TryInto::<IXmlText>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlCDataSection {}
unsafe impl ::core::marker::Sync for XmlCDataSection {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlComment(pub ::windows::core::IInspectable);
impl XmlComment {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlComment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
}
unsafe impl ::windows::core::Interface for XmlComment {
    type Vtable = IXmlComment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
impl ::windows::core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
impl ::core::convert::From<XmlComment> for ::windows::core::IUnknown {
    fn from(value: XmlComment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlComment> for ::windows::core::IUnknown {
    fn from(value: &XmlComment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlComment> for ::windows::core::IInspectable {
    fn from(value: XmlComment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlComment> for ::windows::core::IInspectable {
    fn from(value: &XmlComment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlComment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for &XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlComment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlComment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlComment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlComment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlComment {}
unsafe impl ::core::marker::Sync for XmlComment {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlDocument(pub ::windows::core::IInspectable);
impl XmlDocument {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XmlDocument, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Doctype(&self) -> ::windows::core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocumentType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDomImplementation>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DocumentElement(&self) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tagname: Param0) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocumentFragment>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateTextNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlComment>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, target: Param0, data: Param1) -> ::windows::core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), target.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<XmlProcessingInstruction>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateEntityReference<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<XmlEntityReference>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementsByTagName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tagname: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateCDataSection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<XmlCDataSection>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateAttributeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CreateElementNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, elementid: Param0) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), elementid.into_param().abi(), &mut result__).from_abi::<XmlElement>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ImportNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, node: Param0, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), node.into_param().abi(), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LoadXml<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xml: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xml.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LoadXmlWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, XmlLoadSettings>>(&self, xml: Param0, loadsettings: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xml.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Storage_Streams`*"]
    pub fn LoadXmlFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Storage_Streams`*"]
    pub fn LoadXmlFromBufferWithSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, XmlLoadSettings>>(&self, buffer: Param0, loadsettings: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), buffer.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`*"]
    pub fn LoadFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`*"]
    pub fn LoadFromUriWithSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, XmlLoadSettings>>(uri: Param0, loadsettings: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), loadsettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn LoadFromFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation`, `Storage`*"]
    pub fn LoadFromFileWithSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, XmlLoadSettings>>(file: Param0, loadsettings: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), file.into_param().abi(), loadsettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
}
unsafe impl ::windows::core::Interface for XmlDocument {
    type Vtable = IXmlDocument_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
impl ::windows::core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
impl ::core::convert::From<XmlDocument> for ::windows::core::IUnknown {
    fn from(value: XmlDocument) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows::core::IUnknown {
    fn from(value: &XmlDocument) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlDocument> for ::windows::core::IInspectable {
    fn from(value: XmlDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows::core::IInspectable {
    fn from(value: &XmlDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocument {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocument {}
unsafe impl ::core::marker::Sync for XmlDocument {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlDocumentFragment(pub ::windows::core::IInspectable);
impl XmlDocumentFragment {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
}
unsafe impl ::windows::core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
impl ::windows::core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows::core::IUnknown {
    fn from(value: XmlDocumentFragment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows::core::IUnknown {
    fn from(value: &XmlDocumentFragment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows::core::IInspectable {
    fn from(value: XmlDocumentFragment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows::core::IInspectable {
    fn from(value: &XmlDocumentFragment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocumentFragment {}
unsafe impl ::core::marker::Sync for XmlDocumentFragment {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlDocumentType(pub ::windows::core::IInspectable);
impl XmlDocumentType {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
}
unsafe impl ::windows::core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
impl ::windows::core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
impl ::core::convert::From<XmlDocumentType> for ::windows::core::IUnknown {
    fn from(value: XmlDocumentType) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows::core::IUnknown {
    fn from(value: &XmlDocumentType) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlDocumentType> for ::windows::core::IInspectable {
    fn from(value: XmlDocumentType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows::core::IInspectable {
    fn from(value: &XmlDocumentType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentType {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocumentType {}
unsafe impl ::core::marker::Sync for XmlDocumentType {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlDomImplementation(pub ::windows::core::IInspectable);
impl XmlDomImplementation {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasFeature<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, feature: Param0, version: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), feature.into_param().abi(), version.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
}
unsafe impl ::windows::core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
impl ::windows::core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
impl ::core::convert::From<XmlDomImplementation> for ::windows::core::IUnknown {
    fn from(value: XmlDomImplementation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows::core::IUnknown {
    fn from(value: &XmlDomImplementation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlDomImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlDomImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlDomImplementation> for ::windows::core::IInspectable {
    fn from(value: XmlDomImplementation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows::core::IInspectable {
    fn from(value: &XmlDomImplementation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlDomImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlDomImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XmlDomImplementation {}
unsafe impl ::core::marker::Sync for XmlDomImplementation {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlElement(pub ::windows::core::IInspectable);
impl XmlElement {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn TagName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, attributename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), attributename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, attributename: Param0, attributevalue: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), attributename.into_param().abi(), attributevalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, attributename: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), attributename.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, attributename: Param0) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), attributename.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNode<'a, Param0: ::windows::core::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), newattribute.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttributeNode<'a, Param0: ::windows::core::IntoParam<'a, XmlAttribute>>(&self, attributenode: Param0) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), attributenode.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetElementsByTagName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tagname: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), tagname.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveAttributeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetAttributeNodeNS<'a, Param0: ::windows::core::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), newattribute.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetAttributeNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), localname.into_param().abi(), &mut result__).from_abi::<XmlAttribute>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
}
unsafe impl ::windows::core::Interface for XmlElement {
    type Vtable = IXmlElement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
impl ::windows::core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
impl ::core::convert::From<XmlElement> for ::windows::core::IUnknown {
    fn from(value: XmlElement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlElement> for ::windows::core::IUnknown {
    fn from(value: &XmlElement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlElement> for ::windows::core::IInspectable {
    fn from(value: XmlElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlElement> for ::windows::core::IInspectable {
    fn from(value: &XmlElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlElement {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlElement {}
unsafe impl ::core::marker::Sync for XmlElement {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlEntityReference(pub ::windows::core::IInspectable);
impl XmlEntityReference {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
}
unsafe impl ::windows::core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
impl ::windows::core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
impl ::core::convert::From<XmlEntityReference> for ::windows::core::IUnknown {
    fn from(value: XmlEntityReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows::core::IUnknown {
    fn from(value: &XmlEntityReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlEntityReference> for ::windows::core::IInspectable {
    fn from(value: XmlEntityReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows::core::IInspectable {
    fn from(value: &XmlEntityReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlEntityReference {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlEntityReference {}
unsafe impl ::core::marker::Sync for XmlEntityReference {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlLoadSettings(pub ::windows::core::IInspectable);
impl XmlLoadSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XmlLoadSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn MaxElementDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ProhibitDtd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ResolveExternals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ValidateOnParse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
}
unsafe impl ::windows::core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
impl ::windows::core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
impl ::core::convert::From<XmlLoadSettings> for ::windows::core::IUnknown {
    fn from(value: XmlLoadSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows::core::IUnknown {
    fn from(value: &XmlLoadSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlLoadSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlLoadSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlLoadSettings> for ::windows::core::IInspectable {
    fn from(value: XmlLoadSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows::core::IInspectable {
    fn from(value: &XmlLoadSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlLoadSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlLoadSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XmlLoadSettings {}
unsafe impl ::core::marker::Sync for XmlLoadSettings {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlNamedNodeMap(pub ::windows::core::IInspectable);
impl XmlNamedNodeMap {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetNamedItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNamedItem<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), node.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveNamedItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetNamedItemNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveNamedItemNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), namespaceuri.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNamedItemNS<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), node.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IXmlNode as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
}
unsafe impl ::windows::core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
impl ::windows::core::RuntimeName for XmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows::core::IUnknown {
    fn from(value: XmlNamedNodeMap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows::core::IUnknown {
    fn from(value: &XmlNamedNodeMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows::core::IInspectable {
    fn from(value: XmlNamedNodeMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows::core::IInspectable {
    fn from(value: &XmlNamedNodeMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<XmlNamedNodeMap> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<XmlNamedNodeMap> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlNamedNodeMap {}
unsafe impl ::core::marker::Sync for XmlNamedNodeMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlNodeList(pub ::windows::core::IInspectable);
impl XmlNodeList {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Xml_Dom`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IXmlNode as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
}
unsafe impl ::windows::core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
impl ::windows::core::RuntimeName for XmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
}
impl ::core::convert::From<XmlNodeList> for ::windows::core::IUnknown {
    fn from(value: XmlNodeList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows::core::IUnknown {
    fn from(value: &XmlNodeList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlNodeList> for ::windows::core::IInspectable {
    fn from(value: XmlNodeList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows::core::IInspectable {
    fn from(value: &XmlNodeList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<XmlNodeList> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlNodeList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&XmlNodeList> for super::super::super::Foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<XmlNodeList> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlNodeList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&XmlNodeList> for super::super::super::Foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlNodeList {}
unsafe impl ::core::marker::Sync for XmlNodeList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlProcessingInstruction(pub ::windows::core::IInspectable);
impl XmlProcessingInstruction {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
}
unsafe impl ::windows::core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
impl ::windows::core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows::core::IUnknown {
    fn from(value: XmlProcessingInstruction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows::core::IUnknown {
    fn from(value: &XmlProcessingInstruction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows::core::IInspectable {
    fn from(value: XmlProcessingInstruction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows::core::IInspectable {
    fn from(value: &XmlProcessingInstruction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlProcessingInstruction {}
unsafe impl ::core::marker::Sync for XmlProcessingInstruction {}
#[doc = "*Required features: `Data_Xml_Dom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XmlText(pub ::windows::core::IInspectable);
impl XmlText {
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), offset, &mut result__).from_abi::<IXmlText>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), offset, count, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), offset, count).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceData<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), offset, count, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: NodeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NodeType>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InsertBefore<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn ReplaceChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>, Param1: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), newchild.into_param().abi(), referencechild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn RemoveChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), childnode.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn AppendChild<'a, Param0: ::windows::core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), newchild.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), deep, &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetPrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xpath: Param0) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), xpath.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectSingleNodeNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SelectNodesNS<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi::<XmlNodeList>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Xml_Dom`*"]
    pub fn SetInnerText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for XmlText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
}
unsafe impl ::windows::core::Interface for XmlText {
    type Vtable = IXmlText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
impl ::windows::core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
impl ::core::convert::From<XmlText> for ::windows::core::IUnknown {
    fn from(value: XmlText) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XmlText> for ::windows::core::IUnknown {
    fn from(value: &XmlText) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XmlText> for ::windows::core::IInspectable {
    fn from(value: XmlText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XmlText> for ::windows::core::IInspectable {
    fn from(value: &XmlText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<XmlText> for IXmlText {
    fn from(value: XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlText> for IXmlText {
    fn from(value: &XmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlText> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlText> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlText> for &XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlText> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlCharacterData {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlCharacterData> for &XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNode> for &XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNodeSelector {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSelector> for &XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNodeSerializer {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IXmlNodeSerializer> for &XmlText {
    fn into_param(self) -> ::windows::core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlText {}
unsafe impl ::core::marker::Sync for XmlText {}

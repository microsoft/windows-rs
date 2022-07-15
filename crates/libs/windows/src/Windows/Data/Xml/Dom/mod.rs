#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdEntity(::windows::core::IUnknown);
impl DtdEntity {
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PublicId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SystemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NotationName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for DtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DtdEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DtdEntity {}
impl ::core::fmt::Debug for DtdEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdEntity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
    const IID: ::windows::core::GUID = <IDtdEntity as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
impl ::core::convert::From<DtdEntity> for ::windows::core::IUnknown {
    fn from(value: DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows::core::IUnknown {
    fn from(value: &DtdEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DtdEntity> for &::windows::core::IUnknown {
    fn from(value: &DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DtdEntity> for ::windows::core::IInspectable {
    fn from(value: DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows::core::IInspectable {
    fn from(value: &DtdEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DtdEntity> for &::windows::core::IInspectable {
    fn from(value: &DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdEntity) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DtdEntity {}
unsafe impl ::core::marker::Sync for DtdEntity {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdNotation(::windows::core::IUnknown);
impl DtdNotation {
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PublicId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SystemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for DtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DtdNotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DtdNotation {}
impl ::core::fmt::Debug for DtdNotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdNotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
    const IID: ::windows::core::GUID = <IDtdNotation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
impl ::core::convert::From<DtdNotation> for ::windows::core::IUnknown {
    fn from(value: DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows::core::IUnknown {
    fn from(value: &DtdNotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DtdNotation> for &::windows::core::IUnknown {
    fn from(value: &DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DtdNotation> for ::windows::core::IInspectable {
    fn from(value: DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows::core::IInspectable {
    fn from(value: &DtdNotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DtdNotation> for &::windows::core::IInspectable {
    fn from(value: &DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdEntity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDtdEntity {
    type Vtable = IDtdEntity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdNotation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDtdNotation {
    type Vtable = IDtdNotation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlCDataSection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlCharacterData(::windows::core::IUnknown);
impl IXmlCharacterData {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::convert::From<IXmlCharacterData> for ::windows::core::IUnknown {
    fn from(value: IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlCharacterData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows::core::IUnknown {
    fn from(value: &IXmlCharacterData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXmlCharacterData> for ::windows::core::IInspectable {
    fn from(value: IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlCharacterData> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows::core::IInspectable {
    fn from(value: &IXmlCharacterData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IXmlCharacterData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlCharacterData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlCharacterData {}
impl ::core::fmt::Debug for IXmlCharacterData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlCharacterData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlComment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlComment {
    type Vtable = IXmlComment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocument {
    type Vtable = IXmlDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentFragment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LoadXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loadsettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveToFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBufferWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBufferWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5543d254_d757_4b79_9539_232b18f50bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileWithSettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentType(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDomImplementation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlElement {
    type Vtable = IXmlElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, attributevalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlEntityReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlLoadSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNamedNodeMap(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNode(::windows::core::IUnknown);
impl IXmlNode {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::convert::From<IXmlNode> for ::windows::core::IUnknown {
    fn from(value: IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNode> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows::core::IUnknown {
    fn from(value: &IXmlNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXmlNode> for ::windows::core::IInspectable {
    fn from(value: IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNode> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows::core::IInspectable {
    fn from(value: &IXmlNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IXmlNode> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlNode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlNode> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlNode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IXmlNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNode {}
impl ::core::fmt::Debug for IXmlNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXmlNode {
    type Vtable = IXmlNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows::core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Prefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNodeList(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSelector(::windows::core::IUnknown);
impl IXmlNodeSelector {
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows::core::IUnknown {
    fn from(value: IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNodeSelector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows::core::IUnknown {
    fn from(value: &IXmlNodeSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows::core::IInspectable {
    fn from(value: IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNodeSelector> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows::core::IInspectable {
    fn from(value: &IXmlNodeSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXmlNodeSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNodeSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNodeSelector {}
impl ::core::fmt::Debug for IXmlNodeSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNodeSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSerializer(::windows::core::IUnknown);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows::core::IUnknown {
    fn from(value: IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNodeSerializer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows::core::IUnknown {
    fn from(value: &IXmlNodeSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows::core::IInspectable {
    fn from(value: IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlNodeSerializer> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows::core::IInspectable {
    fn from(value: &IXmlNodeSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXmlNodeSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNodeSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNodeSerializer {}
impl ::core::fmt::Debug for IXmlNodeSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNodeSerializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlProcessingInstruction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlText(::windows::core::IUnknown);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::convert::From<IXmlText> for ::windows::core::IUnknown {
    fn from(value: IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlText> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlText> for ::windows::core::IUnknown {
    fn from(value: &IXmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXmlText> for ::windows::core::IInspectable {
    fn from(value: IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXmlText> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlText> for ::windows::core::IInspectable {
    fn from(value: &IXmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<'a, IXmlCharacterData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IXmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlText {}
impl ::core::fmt::Debug for IXmlText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXmlText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXmlText {
    type Vtable = IXmlText_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SplitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NodeType(pub i32);
impl NodeType {
    pub const Invalid: Self = Self(0i32);
    pub const ElementNode: Self = Self(1i32);
    pub const AttributeNode: Self = Self(2i32);
    pub const TextNode: Self = Self(3i32);
    pub const DataSectionNode: Self = Self(4i32);
    pub const EntityReferenceNode: Self = Self(5i32);
    pub const EntityNode: Self = Self(6i32);
    pub const ProcessingInstructionNode: Self = Self(7i32);
    pub const CommentNode: Self = Self(8i32);
    pub const DocumentNode: Self = Self(9i32);
    pub const DocumentTypeNode: Self = Self(10i32);
    pub const DocumentFragmentNode: Self = Self(11i32);
    pub const NotationNode: Self = Self(12i32);
}
impl ::core::marker::Copy for NodeType {}
impl ::core::clone::Clone for NodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NodeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NodeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlAttribute(::windows::core::IUnknown);
impl XmlAttribute {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Specified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Specified)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlAttribute {}
impl ::core::fmt::Debug for XmlAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
    const IID: ::windows::core::GUID = <IXmlAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
impl ::core::convert::From<XmlAttribute> for ::windows::core::IUnknown {
    fn from(value: XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows::core::IUnknown {
    fn from(value: &XmlAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlAttribute> for &::windows::core::IUnknown {
    fn from(value: &XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlAttribute> for ::windows::core::IInspectable {
    fn from(value: XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows::core::IInspectable {
    fn from(value: &XmlAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlAttribute> for &::windows::core::IInspectable {
    fn from(value: &XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlAttribute {}
unsafe impl ::core::marker::Sync for XmlAttribute {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlCDataSection(::windows::core::IUnknown);
impl XmlCDataSection {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = &::windows::core::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlCDataSection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlCDataSection {}
impl ::core::fmt::Debug for XmlCDataSection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlCDataSection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
    const IID: ::windows::core::GUID = <IXmlCDataSection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
impl ::core::convert::From<XmlCDataSection> for ::windows::core::IUnknown {
    fn from(value: XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows::core::IUnknown {
    fn from(value: &XmlCDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlCDataSection> for &::windows::core::IUnknown {
    fn from(value: &XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlCDataSection> for ::windows::core::IInspectable {
    fn from(value: XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows::core::IInspectable {
    fn from(value: &XmlCDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlCDataSection> for &::windows::core::IInspectable {
    fn from(value: &XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<'a, IXmlCharacterData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<'a, IXmlText> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlCDataSection {}
unsafe impl ::core::marker::Sync for XmlCDataSection {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlComment(::windows::core::IUnknown);
impl XmlComment {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlComment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlComment {}
impl ::core::fmt::Debug for XmlComment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlComment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlComment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlComment {
    type Vtable = IXmlComment_Vtbl;
    const IID: ::windows::core::GUID = <IXmlComment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
impl ::core::convert::From<XmlComment> for ::windows::core::IUnknown {
    fn from(value: XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlComment> for ::windows::core::IUnknown {
    fn from(value: &XmlComment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlComment> for &::windows::core::IUnknown {
    fn from(value: &XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlComment> for ::windows::core::IInspectable {
    fn from(value: XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlComment> for ::windows::core::IInspectable {
    fn from(value: &XmlComment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlComment> for &::windows::core::IInspectable {
    fn from(value: &XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<'a, IXmlCharacterData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlComment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlComment {}
unsafe impl ::core::marker::Sync for XmlComment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocument(::windows::core::IUnknown);
impl XmlDocument {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XmlDocument, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Doctype(&self) -> ::windows::core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Doctype)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocumentType>(result__)
        }
    }
    pub fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Implementation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDomImplementation>(result__)
        }
    }
    pub fn DocumentElement(&self) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentElement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn CreateElement(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateElement)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDocumentFragment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocumentFragment>(result__)
        }
    }
    pub fn CreateTextNode(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateTextNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi::<XmlText>(result__)
        }
    }
    pub fn CreateComment(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateComment)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi::<XmlComment>(result__)
        }
    }
    pub fn CreateProcessingInstruction(&self, target: &::windows::core::HSTRING, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateProcessingInstruction)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(target), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi::<XmlProcessingInstruction>(result__)
        }
    }
    pub fn CreateAttribute(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn CreateEntityReference(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEntityReference)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<XmlEntityReference>(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetElementsByTagName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn CreateCDataSection(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCDataSection)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi::<XmlCDataSection>(result__)
        }
    }
    pub fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateAttributeNS<'a, P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn CreateElementNS<'a, P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateElementNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn GetElementById(&self, elementid: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetElementById)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(elementid), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn ImportNode<'a, P0, E0>(&self, node: P0, deep: bool) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportNode)(::windows::core::Interface::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LoadXml(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXml)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xml)).ok() }
    }
    pub fn LoadXmlWithSettings<'a, P0>(&self, xml: &::windows::core::HSTRING, loadsettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XmlLoadSettings>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlWithSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xml), loadsettings.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveToFileAsync<'a, P0, E0>(&self, file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::windows::core::Interface::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBuffer<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlFromBuffer)(::windows::core::Interface::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBufferWithSettings<'a, P0, E0, P1>(&self, buffer: P0, loadsettings: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, XmlLoadSettings>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlFromBufferWithSettings)(::windows::core::Interface::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), loadsettings.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriAsync<'a, P0>(uri: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::Uri>>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromUriAsync)(::windows::core::Interface::as_raw(this), uri.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriWithSettingsAsync<'a, P0, P1>(uri: P0, loadsettings: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::Uri>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, XmlLoadSettings>>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromUriWithSettingsAsync)(::windows::core::Interface::as_raw(this), uri.into().abi(), loadsettings.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileAsync<'a, P0, E0>(file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFileAsync)(::windows::core::Interface::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileWithSettingsAsync<'a, P0, E0, P1>(file: P0, loadsettings: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, XmlLoadSettings>>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFileWithSettingsAsync)(::windows::core::Interface::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), loadsettings.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc(hidden)]
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocument {}
impl ::core::fmt::Debug for XmlDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
    const IID: ::windows::core::GUID = <IXmlDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
impl ::core::convert::From<XmlDocument> for ::windows::core::IUnknown {
    fn from(value: XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows::core::IUnknown {
    fn from(value: &XmlDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocument> for &::windows::core::IUnknown {
    fn from(value: &XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlDocument> for ::windows::core::IInspectable {
    fn from(value: XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows::core::IInspectable {
    fn from(value: &XmlDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocument> for &::windows::core::IInspectable {
    fn from(value: &XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlDocument {}
unsafe impl ::core::marker::Sync for XmlDocument {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentFragment(::windows::core::IUnknown);
impl XmlDocumentFragment {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocumentFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocumentFragment {}
impl ::core::fmt::Debug for XmlDocumentFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocumentFragment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
    const IID: ::windows::core::GUID = <IXmlDocumentFragment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows::core::IUnknown {
    fn from(value: XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows::core::IUnknown {
    fn from(value: &XmlDocumentFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for &::windows::core::IUnknown {
    fn from(value: &XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows::core::IInspectable {
    fn from(value: XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows::core::IInspectable {
    fn from(value: &XmlDocumentFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for &::windows::core::IInspectable {
    fn from(value: &XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlDocumentFragment {}
unsafe impl ::core::marker::Sync for XmlDocumentFragment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentType(::windows::core::IUnknown);
impl XmlDocumentType {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Entities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Notations)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocumentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocumentType {}
impl ::core::fmt::Debug for XmlDocumentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocumentType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
    const IID: ::windows::core::GUID = <IXmlDocumentType as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
impl ::core::convert::From<XmlDocumentType> for ::windows::core::IUnknown {
    fn from(value: XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows::core::IUnknown {
    fn from(value: &XmlDocumentType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocumentType> for &::windows::core::IUnknown {
    fn from(value: &XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlDocumentType> for ::windows::core::IInspectable {
    fn from(value: XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows::core::IInspectable {
    fn from(value: &XmlDocumentType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDocumentType> for &::windows::core::IInspectable {
    fn from(value: &XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlDocumentType {}
unsafe impl ::core::marker::Sync for XmlDocumentType {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDomImplementation(::windows::core::IUnknown);
impl XmlDomImplementation {
    pub fn HasFeature<'a, P0>(&self, feature: &::windows::core::HSTRING, version: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasFeature)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(feature), version.into().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDomImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDomImplementation {}
impl ::core::fmt::Debug for XmlDomImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDomImplementation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
    const IID: ::windows::core::GUID = <IXmlDomImplementation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
impl ::core::convert::From<XmlDomImplementation> for ::windows::core::IUnknown {
    fn from(value: XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows::core::IUnknown {
    fn from(value: &XmlDomImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDomImplementation> for &::windows::core::IUnknown {
    fn from(value: &XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlDomImplementation> for ::windows::core::IInspectable {
    fn from(value: XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows::core::IInspectable {
    fn from(value: &XmlDomImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlDomImplementation> for &::windows::core::IInspectable {
    fn from(value: &XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XmlDomImplementation {}
unsafe impl ::core::marker::Sync for XmlDomImplementation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlElement(::windows::core::IUnknown);
impl XmlElement {
    pub fn TagName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TagName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAttribute(&self, attributename: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), ::core::mem::transmute_copy(attributevalue)).ok() }
    }
    pub fn RemoveAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename)).ok() }
    }
    pub fn GetAttributeNode(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributeNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn SetAttributeNode<'a, P0>(&self, newattribute: P0) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XmlAttribute>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SetAttributeNode)(::windows::core::Interface::as_raw(this), newattribute.into().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn RemoveAttributeNode<'a, P0>(&self, attributenode: P0) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XmlAttribute>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAttributeNode)(::windows::core::Interface::as_raw(this), attributenode.into().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetElementsByTagName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SetAttributeNS<'a, P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetAttributeNS<'a, P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RemoveAttributeNS<'a, P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname)).ok() }
    }
    pub fn SetAttributeNodeNS<'a, P0>(&self, newattribute: P0) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XmlAttribute>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SetAttributeNodeNS)(::windows::core::Interface::as_raw(this), newattribute.into().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn GetAttributeNodeNS<'a, P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributeNodeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlElement {}
impl ::core::fmt::Debug for XmlElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlElement {
    type Vtable = IXmlElement_Vtbl;
    const IID: ::windows::core::GUID = <IXmlElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
impl ::core::convert::From<XmlElement> for ::windows::core::IUnknown {
    fn from(value: XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlElement> for ::windows::core::IUnknown {
    fn from(value: &XmlElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlElement> for &::windows::core::IUnknown {
    fn from(value: &XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlElement> for ::windows::core::IInspectable {
    fn from(value: XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlElement> for ::windows::core::IInspectable {
    fn from(value: &XmlElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlElement> for &::windows::core::IInspectable {
    fn from(value: &XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlElement {}
unsafe impl ::core::marker::Sync for XmlElement {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlEntityReference(::windows::core::IUnknown);
impl XmlEntityReference {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlEntityReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlEntityReference {}
impl ::core::fmt::Debug for XmlEntityReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlEntityReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
    const IID: ::windows::core::GUID = <IXmlEntityReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
impl ::core::convert::From<XmlEntityReference> for ::windows::core::IUnknown {
    fn from(value: XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows::core::IUnknown {
    fn from(value: &XmlEntityReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlEntityReference> for &::windows::core::IUnknown {
    fn from(value: &XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlEntityReference> for ::windows::core::IInspectable {
    fn from(value: XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows::core::IInspectable {
    fn from(value: &XmlEntityReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlEntityReference> for &::windows::core::IInspectable {
    fn from(value: &XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlEntityReference {}
unsafe impl ::core::marker::Sync for XmlEntityReference {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlLoadSettings(::windows::core::IUnknown);
impl XmlLoadSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XmlLoadSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxElementDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxElementDepth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxElementDepth)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProhibitDtd)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProhibitDtd)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ResolveExternals)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResolveExternals)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidateOnParse)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValidateOnParse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ElementContentWhiteSpace)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetElementContentWhiteSpace)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for XmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlLoadSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlLoadSettings {}
impl ::core::fmt::Debug for XmlLoadSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlLoadSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
    const IID: ::windows::core::GUID = <IXmlLoadSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
impl ::core::convert::From<XmlLoadSettings> for ::windows::core::IUnknown {
    fn from(value: XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows::core::IUnknown {
    fn from(value: &XmlLoadSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlLoadSettings> for &::windows::core::IUnknown {
    fn from(value: &XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlLoadSettings> for ::windows::core::IInspectable {
    fn from(value: XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows::core::IInspectable {
    fn from(value: &XmlLoadSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlLoadSettings> for &::windows::core::IInspectable {
    fn from(value: &XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XmlLoadSettings {}
unsafe impl ::core::marker::Sync for XmlLoadSettings {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNamedNodeMap(::windows::core::IUnknown);
impl XmlNamedNodeMap {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Item)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn GetNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SetNamedItem<'a, P0, E0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SetNamedItem)(::windows::core::Interface::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveNamedItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn GetNamedItemNS<'a, P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedItemNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveNamedItemNS<'a, P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveNamedItemNS)(::windows::core::Interface::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SetNamedItemNS<'a, P0, E0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SetNamedItemNS)(::windows::core::Interface::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlNamedNodeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlNamedNodeMap {}
impl ::core::fmt::Debug for XmlNamedNodeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNamedNodeMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
    const IID: ::windows::core::GUID = <IXmlNamedNodeMap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows::core::IUnknown {
    fn from(value: XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows::core::IUnknown {
    fn from(value: &XmlNamedNodeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for &::windows::core::IUnknown {
    fn from(value: &XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows::core::IInspectable {
    fn from(value: XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows::core::IInspectable {
    fn from(value: &XmlNamedNodeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for &::windows::core::IInspectable {
    fn from(value: &XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlNamedNodeMap> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlNamedNodeMap> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlNamedNodeMap {}
unsafe impl ::core::marker::Sync for XmlNamedNodeMap {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNodeList(::windows::core::IUnknown);
impl XmlNodeList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Item)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlNodeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlNodeList {}
impl ::core::fmt::Debug for XmlNodeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNodeList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
    const IID: ::windows::core::GUID = <IXmlNodeList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<XmlNodeList> for ::windows::core::IUnknown {
    fn from(value: XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows::core::IUnknown {
    fn from(value: &XmlNodeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlNodeList> for &::windows::core::IUnknown {
    fn from(value: &XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlNodeList> for ::windows::core::IInspectable {
    fn from(value: XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows::core::IInspectable {
    fn from(value: &XmlNodeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlNodeList> for &::windows::core::IInspectable {
    fn from(value: &XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlNodeList> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlNodeList> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlNodeList {}
unsafe impl ::core::marker::Sync for XmlNodeList {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlProcessingInstruction(::windows::core::IUnknown);
impl XmlProcessingInstruction {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Target)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for XmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlProcessingInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlProcessingInstruction {}
impl ::core::fmt::Debug for XmlProcessingInstruction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlProcessingInstruction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
    const IID: ::windows::core::GUID = <IXmlProcessingInstruction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows::core::IUnknown {
    fn from(value: XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows::core::IUnknown {
    fn from(value: &XmlProcessingInstruction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for &::windows::core::IUnknown {
    fn from(value: &XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows::core::IInspectable {
    fn from(value: XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows::core::IInspectable {
    fn from(value: &XmlProcessingInstruction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for &::windows::core::IInspectable {
    fn from(value: &XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlProcessingInstruction {}
unsafe impl ::core::marker::Sync for XmlProcessingInstruction {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlText(::windows::core::IUnknown);
impl XmlText {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlText {}
impl ::core::fmt::Debug for XmlText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XmlText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XmlText {
    type Vtable = IXmlText_Vtbl;
    const IID: ::windows::core::GUID = <IXmlText as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
impl ::core::convert::From<XmlText> for ::windows::core::IUnknown {
    fn from(value: XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlText> for ::windows::core::IUnknown {
    fn from(value: &XmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlText> for &::windows::core::IUnknown {
    fn from(value: &XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XmlText> for ::windows::core::IInspectable {
    fn from(value: XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlText> for ::windows::core::IInspectable {
    fn from(value: &XmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XmlText> for &::windows::core::IInspectable {
    fn from(value: &XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<'a, IXmlCharacterData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<'a, IXmlNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<'a, IXmlNodeSelector> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<'a, IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlText {
    type Error = ::windows::core::Error;
    fn try_from(value: XmlText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlText {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<'a, IXmlText> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlText {}
unsafe impl ::core::marker::Sync for XmlText {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

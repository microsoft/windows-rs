#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdEntity(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
unsafe impl ::windows::core::Interface for IDtdEntity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdNotation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
unsafe impl ::windows::core::Interface for IDtdNotation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlCDataSection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlCDataSection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlCharacterData(::windows::core::IUnknown);
impl IXmlCharacterData {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubstringData)(::windows::core::Vtable::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AppendData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InsertData)(::windows::core::Vtable::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteData)(::windows::core::Vtable::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceData)(::windows::core::Vtable::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::core::interface_hierarchy!(IXmlCharacterData, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&IXmlCharacterData> for ::windows::core::InParam<IXmlNodeSerializer> {
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
unsafe impl ::windows::core::Vtable for IXmlCharacterData {
    type Vtable = IXmlCharacterData_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlCharacterData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlComment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlComment {
    type Vtable = IXmlComment_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlComment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentFragment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocumentFragment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocumentIO {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LoadXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveToFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocumentIO2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
unsafe impl ::windows::core::Vtable for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocumentStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5543d254_d757_4b79_9539_232b18f50bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
unsafe impl ::windows::core::Vtable for IXmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDocumentType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDomImplementation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlDomImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: *mut ::core::ffi::c_void, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlElement {
    type Vtable = IXmlElement_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::core::ffi::c_void, attributevalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlEntityReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlEntityReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlLoadSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlLoadSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
unsafe impl ::windows::core::Vtable for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlNamedNodeMap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::core::interface_hierarchy!(IXmlNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&IXmlNode> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&IXmlNode> for ::windows::core::InParam<IXmlNodeSerializer> {
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
unsafe impl ::windows::core::Vtable for IXmlNode {
    type Vtable = IXmlNode_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows::core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IXmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlNodeList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IXmlNodeSelector, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlNodeSelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: *mut ::core::ffi::c_void, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: *mut ::core::ffi::c_void, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSerializer(::windows::core::IUnknown);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::core::interface_hierarchy!(IXmlNodeSerializer, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlNodeSerializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlProcessingInstruction(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlProcessingInstruction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlText(::windows::core::IUnknown);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SplitText)(::windows::core::Vtable::as_raw(this), offset, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubstringData)(::windows::core::Vtable::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AppendData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertData)(::windows::core::Vtable::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteData)(::windows::core::Vtable::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceData)(::windows::core::Vtable::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::core::interface_hierarchy!(IXmlText, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<IXmlCharacterData> {
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
impl ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&IXmlText> for ::windows::core::InParam<IXmlNodeSerializer> {
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
unsafe impl ::windows::core::Vtable for IXmlText {
    type Vtable = IXmlText_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SplitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdEntity(::windows::core::IUnknown);
impl DtdEntity {
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublicId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotationName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
unsafe impl ::windows::core::Interface for DtdEntity {
    const IID: ::windows::core::GUID = <IDtdEntity as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
::windows::core::interface_hierarchy!(DtdEntity, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&DtdEntity> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).PublicId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
unsafe impl ::windows::core::Interface for DtdNotation {
    const IID: ::windows::core::GUID = <IDtdNotation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
::windows::core::interface_hierarchy!(DtdNotation, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&DtdNotation> for ::windows::core::InParam<IXmlNodeSerializer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DtdNotation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlAttribute(::windows::core::IUnknown);
impl XmlAttribute {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Specified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Specified)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlAttribute {
    const IID: ::windows::core::GUID = <IXmlAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
::windows::core::interface_hierarchy!(XmlAttribute, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlAttribute> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubstringData)(::windows::core::Vtable::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AppendData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertData)(::windows::core::Vtable::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteData)(::windows::core::Vtable::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceData)(::windows::core::Vtable::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = &::windows::core::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SplitText)(::windows::core::Vtable::as_raw(this), offset, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlCDataSection {
    const IID: ::windows::core::GUID = <IXmlCDataSection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
::windows::core::interface_hierarchy!(XmlCDataSection, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<IXmlCharacterData> {
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
impl ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<IXmlNodeSerializer> {
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
impl ::core::convert::TryFrom<&XmlCDataSection> for ::windows::core::InParam<IXmlText> {
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
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubstringData)(::windows::core::Vtable::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AppendData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertData)(::windows::core::Vtable::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteData)(::windows::core::Vtable::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceData)(::windows::core::Vtable::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlComment {
    type Vtable = IXmlComment_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlComment {
    const IID: ::windows::core::GUID = <IXmlComment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
::windows::core::interface_hierarchy!(XmlComment, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<IXmlCharacterData> {
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
impl ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlComment> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).Doctype)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Implementation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DocumentElement(&self) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentElement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateElement(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateElement)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDocumentFragment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateTextNode(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTextNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateComment(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateProcessingInstruction(&self, target: &::windows::core::HSTRING, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateProcessingInstruction)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(target), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateAttribute(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAttribute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateEntityReference(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEntityReference)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetElementsByTagName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateCDataSection(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCDataSection)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAttributeNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateElementNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateElementNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetElementById(&self, elementid: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetElementById)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(elementid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ImportNode<P0, E0>(&self, node: P0, deep: bool) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportNode)(::windows::core::Vtable::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LoadXml(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).LoadXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xml)).ok() }
    }
    pub fn LoadXmlWithSettings(&self, xml: &::windows::core::HSTRING, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).LoadXmlWithSettings)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xml), ::core::mem::transmute_copy(loadsettings)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveToFileAsync<P0, E0>(&self, file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveToFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBuffer<P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).LoadXmlFromBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBufferWithSettings<P0, E0>(&self, buffer: P0, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).LoadXmlFromBufferWithSettings)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(loadsettings)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriAsync(uri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadFromUriAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriWithSettingsAsync(uri: &super::super::super::Foundation::Uri, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadFromUriWithSettingsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(loadsettings), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileAsync<P0, E0>(file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadFromFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileWithSettingsAsync<P0, E0>(file: P0, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadFromFileWithSettingsAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(loadsettings), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlDocument {
    const IID: ::windows::core::GUID = <IXmlDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
::windows::core::interface_hierarchy!(XmlDocument, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlDocument> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlDocumentFragment {
    const IID: ::windows::core::GUID = <IXmlDocumentFragment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
::windows::core::interface_hierarchy!(XmlDocumentFragment, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlDocumentFragment> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Entities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlDocumentType {
    const IID: ::windows::core::GUID = <IXmlDocumentType as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
::windows::core::interface_hierarchy!(XmlDocumentType, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlDocumentType> for ::windows::core::InParam<IXmlNodeSerializer> {
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
    pub fn HasFeature<P0>(&self, feature: &::windows::core::HSTRING, version: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasFeature)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feature), version.into().abi(), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlDomImplementation {
    const IID: ::windows::core::GUID = <IXmlDomImplementation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
::windows::core::interface_hierarchy!(XmlDomImplementation, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            (::windows::core::Vtable::vtable(this).TagName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttribute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributename), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAttribute(&self, attributename: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAttribute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributename), ::core::mem::transmute_copy(attributevalue)).ok() }
    }
    pub fn RemoveAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAttribute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributename)).ok() }
    }
    pub fn GetAttributeNode(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttributeNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributename), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAttributeNode(&self, newattribute: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAttributeNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(newattribute), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNode(&self, attributenode: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveAttributeNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributenode), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetElementsByTagName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tagname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAttributeNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(qualifiedname), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttributeNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAttributeNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname)).ok() }
    }
    pub fn SetAttributeNodeNS(&self, newattribute: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAttributeNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(newattribute), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAttributeNodeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttributeNodeNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(localname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlElement {
    type Vtable = IXmlElement_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlElement {
    const IID: ::windows::core::GUID = <IXmlElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
::windows::core::interface_hierarchy!(XmlElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlElement> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlEntityReference {
    const IID: ::windows::core::GUID = <IXmlEntityReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
::windows::core::interface_hierarchy!(XmlEntityReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlEntityReference> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).MaxElementDepth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxElementDepth)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProhibitDtd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProhibitDtd)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolveExternals)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResolveExternals)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateOnParse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValidateOnParse)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementContentWhiteSpace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetElementContentWhiteSpace)(::windows::core::Vtable::as_raw(this), value).ok() }
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
unsafe impl ::windows::core::Vtable for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlLoadSettings {
    const IID: ::windows::core::GUID = <IXmlLoadSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
::windows::core::interface_hierarchy!(XmlLoadSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Item)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNamedItem<P0, E0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetNamedItem)(::windows::core::Vtable::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveNamedItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedItemNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveNamedItemNS)(::windows::core::Vtable::as_raw(this), namespaceuri.into().abi(), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNamedItemNS<P0, E0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetNamedItemNS)(::windows::core::Vtable::as_raw(this), node.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlNamedNodeMap {
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
::windows::core::interface_hierarchy!(XmlNamedNodeMap, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
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
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
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
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Item)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlNodeList {
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
::windows::core::interface_hierarchy!(XmlNodeList, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlNodeList> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<IXmlNode>> {
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
impl ::core::convert::TryFrom<&XmlNodeList> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> {
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
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Target)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
unsafe impl ::windows::core::Vtable for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlProcessingInstruction {
    const IID: ::windows::core::GUID = <IXmlProcessingInstruction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
::windows::core::interface_hierarchy!(XmlProcessingInstruction, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for ::windows::core::InParam<IXmlNodeSerializer> {
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
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubstringData)(::windows::core::Vtable::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AppendData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertData)(::windows::core::Vtable::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteData)(::windows::core::Vtable::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceData)(::windows::core::Vtable::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentNode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextSibling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasChildNodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OwnerDocument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertBefore)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), referencechild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0, E0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveChild)(::windows::core::Vtable::as_raw(this), childnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppendChild)(::windows::core::Vtable::as_raw(this), newchild.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneNode)(::windows::core::Vtable::as_raw(this), deep, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Prefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Normalize)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrefix)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectSingleNodeNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectNodesNS)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InnerText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInnerText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SplitText)(::windows::core::Vtable::as_raw(this), offset, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for XmlText {
    type Vtable = IXmlText_Vtbl;
}
unsafe impl ::windows::core::Interface for XmlText {
    const IID: ::windows::core::GUID = <IXmlText as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
::windows::core::interface_hierarchy!(XmlText, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<IXmlCharacterData> {
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
impl ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<IXmlNode> {
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
impl ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<IXmlNodeSelector> {
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
impl ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<IXmlNodeSerializer> {
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
impl ::core::convert::TryFrom<&XmlText> for ::windows::core::InParam<IXmlText> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XmlText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XmlText {}
unsafe impl ::core::marker::Sync for XmlText {}
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");

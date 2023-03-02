#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdEntity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
impl ::core::clone::Clone for IDtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDtdEntity {
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
unsafe impl ::windows::core::Interface for IDtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
impl ::core::clone::Clone for IDtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDtdNotation {
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
unsafe impl ::windows::core::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
impl ::core::clone::Clone for IXmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlCDataSection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
impl ::core::clone::Clone for IXmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlCDataSection {
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
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
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
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IXmlCharacterData, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IXmlNode> for IXmlCharacterData {}
impl windows::core::CanTryInto<IXmlNodeSelector> for IXmlCharacterData {}
impl windows::core::CanTryInto<IXmlNodeSerializer> for IXmlCharacterData {}
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
impl ::windows::core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
}
unsafe impl ::windows::core::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_Vtbl;
}
impl ::core::clone::Clone for IXmlCharacterData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlCharacterData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlComment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlComment {
    type Vtable = IXmlComment_Vtbl;
}
impl ::core::clone::Clone for IXmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlComment {
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
unsafe impl ::windows::core::Interface for IXmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
impl ::core::clone::Clone for IXmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentFragment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocumentFragment {
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
unsafe impl ::windows::core::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentIO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocumentIO {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LoadXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows::core::HSTRING>, loadsettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IXmlDocumentIO2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocumentIO2 {
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
unsafe impl ::windows::core::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocumentStatics {
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
unsafe impl ::windows::core::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDocumentType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDomImplementation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
impl ::core::clone::Clone for IXmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlDomImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows::core::HSTRING>, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlElement {
    type Vtable = IXmlElement_Vtbl;
}
impl ::core::clone::Clone for IXmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, attributevalue: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlEntityReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
impl ::core::clone::Clone for IXmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlEntityReference {
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
unsafe impl ::windows::core::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
impl ::core::clone::Clone for IXmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlLoadSettings {
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
unsafe impl ::windows::core::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
impl ::core::clone::Clone for IXmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlNamedNodeMap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNode(::windows::core::IUnknown);
impl IXmlNode {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IXmlNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IXmlNodeSelector> for IXmlNode {}
impl windows::core::CanTryInto<IXmlNodeSerializer> for IXmlNode {}
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
impl ::windows::core::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
}
unsafe impl ::windows::core::Interface for IXmlNode {
    type Vtable = IXmlNode_Vtbl;
}
impl ::core::clone::Clone for IXmlNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows::core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IXmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlNodeList {
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
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IXmlNodeSelector, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
}
unsafe impl ::windows::core::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_Vtbl;
}
impl ::core::clone::Clone for IXmlNodeSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlNodeSelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSerializer(::windows::core::IUnknown);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IXmlNodeSerializer, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
}
unsafe impl ::windows::core::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_Vtbl;
}
impl ::core::clone::Clone for IXmlNodeSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlNodeSerializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlProcessingInstruction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
impl ::core::clone::Clone for IXmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlProcessingInstruction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlText(::windows::core::IUnknown);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlText>();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IXmlText, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IXmlCharacterData> for IXmlText {}
impl windows::core::CanTryInto<IXmlNode> for IXmlText {}
impl windows::core::CanTryInto<IXmlNodeSelector> for IXmlText {}
impl windows::core::CanTryInto<IXmlNodeSerializer> for IXmlText {}
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
impl ::windows::core::RuntimeType for IXmlText {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
}
unsafe impl ::windows::core::Interface for IXmlText {
    type Vtable = IXmlText_Vtbl;
}
impl ::core::clone::Clone for IXmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXmlText {
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
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).PublicId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).SystemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NotationName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
}
impl ::core::clone::Clone for DtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DtdEntity {
    const IID: ::windows::core::GUID = <IDtdEntity as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
::windows::imp::interface_hierarchy!(DtdEntity, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for DtdEntity {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for DtdEntity {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for DtdEntity {}
unsafe impl ::core::marker::Send for DtdEntity {}
unsafe impl ::core::marker::Sync for DtdEntity {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdNotation(::windows::core::IUnknown);
impl DtdNotation {
    pub fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).PublicId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).SystemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
}
impl ::core::clone::Clone for DtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DtdNotation {
    const IID: ::windows::core::GUID = <IDtdNotation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
::windows::imp::interface_hierarchy!(DtdNotation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for DtdNotation {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for DtdNotation {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for DtdNotation {}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlAttribute(::windows::core::IUnknown);
impl XmlAttribute {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Specified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Specified)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
}
impl ::core::clone::Clone for XmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlAttribute {
    const IID: ::windows::core::GUID = <IXmlAttribute as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
::windows::imp::interface_hierarchy!(XmlAttribute, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlAttribute {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlAttribute {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlAttribute {}
unsafe impl ::core::marker::Send for XmlAttribute {}
unsafe impl ::core::marker::Sync for XmlAttribute {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlCDataSection(::windows::core::IUnknown);
impl XmlCDataSection {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = &::windows::core::ComInterface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlText>();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
}
impl ::core::clone::Clone for XmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlCDataSection {
    const IID: ::windows::core::GUID = <IXmlCDataSection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
::windows::imp::interface_hierarchy!(XmlCDataSection, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlCharacterData> for XmlCDataSection {}
impl ::windows::core::CanTryInto<IXmlNode> for XmlCDataSection {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlCDataSection {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlCDataSection {}
impl ::windows::core::CanTryInto<IXmlText> for XmlCDataSection {}
unsafe impl ::core::marker::Send for XmlCDataSection {}
unsafe impl ::core::marker::Sync for XmlCDataSection {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlComment(::windows::core::IUnknown);
impl XmlComment {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlComment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
}
impl ::core::clone::Clone for XmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlComment {
    type Vtable = IXmlComment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlComment {
    const IID: ::windows::core::GUID = <IXmlComment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
::windows::imp::interface_hierarchy!(XmlComment, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlCharacterData> for XmlComment {}
impl ::windows::core::CanTryInto<IXmlNode> for XmlComment {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlComment {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlComment {}
unsafe impl ::core::marker::Send for XmlComment {}
unsafe impl ::core::marker::Sync for XmlComment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocument(::windows::core::IUnknown);
impl XmlDocument {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<XmlDocument, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Doctype(&self) -> ::windows::core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocumentType>();
            (::windows::core::Interface::vtable(this).Doctype)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDomImplementation>();
            (::windows::core::Interface::vtable(this).Implementation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentElement(&self) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlElement>();
            (::windows::core::Interface::vtable(this).DocumentElement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateElement(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlElement>();
            (::windows::core::Interface::vtable(this).CreateElement)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocumentFragment>();
            (::windows::core::Interface::vtable(this).CreateDocumentFragment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTextNode(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlText>();
            (::windows::core::Interface::vtable(this).CreateTextNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateComment(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlComment>();
            (::windows::core::Interface::vtable(this).CreateComment)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateProcessingInstruction(&self, target: &::windows::core::HSTRING, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlProcessingInstruction>();
            (::windows::core::Interface::vtable(this).CreateProcessingInstruction)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(target), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAttribute(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).CreateAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateEntityReference(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlEntityReference>();
            (::windows::core::Interface::vtable(this).CreateEntityReference)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).GetElementsByTagName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateCDataSection(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlCDataSection>();
            (::windows::core::Interface::vtable(this).CreateCDataSection)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DocumentUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).CreateAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateElementNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlElement>();
            (::windows::core::Interface::vtable(this).CreateElementNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementById(&self, elementid: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlElement>();
            (::windows::core::Interface::vtable(this).GetElementById)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(elementid), &mut result__).from_abi(result__)
        }
    }
    pub fn ImportNode<P0>(&self, node: P0, deep: bool) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ImportNode)(::windows::core::Interface::as_raw(this), node.try_into_param()?.abi(), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn LoadXml(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXml)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xml)).ok() }
    }
    pub fn LoadXmlWithSettings(&self, xml: &::windows::core::HSTRING, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlWithSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xml), ::core::mem::transmute_copy(loadsettings)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveToFileAsync<P0>(&self, file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBuffer<P0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlFromBuffer)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBufferWithSettings<P0>(&self, buffer: P0, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LoadXmlFromBufferWithSettings)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi(), ::core::mem::transmute_copy(loadsettings)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriAsync(uri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>();
            (::windows::core::Interface::vtable(this).LoadFromUriAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriWithSettingsAsync(uri: &super::super::super::Foundation::Uri, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>();
            (::windows::core::Interface::vtable(this).LoadFromUriWithSettingsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(loadsettings), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileAsync<P0>(file: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>();
            (::windows::core::Interface::vtable(this).LoadFromFileAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileWithSettingsAsync<P0>(file: P0, loadsettings: &XmlLoadSettings) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<XmlDocument>>();
            (::windows::core::Interface::vtable(this).LoadFromFileWithSettingsAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(loadsettings), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc(hidden)]
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
}
impl ::core::clone::Clone for XmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlDocument {
    const IID: ::windows::core::GUID = <IXmlDocument as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
::windows::imp::interface_hierarchy!(XmlDocument, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlDocument {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlDocument {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlDocument {}
unsafe impl ::core::marker::Send for XmlDocument {}
unsafe impl ::core::marker::Sync for XmlDocument {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentFragment(::windows::core::IUnknown);
impl XmlDocumentFragment {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
}
impl ::core::clone::Clone for XmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlDocumentFragment {
    const IID: ::windows::core::GUID = <IXmlDocumentFragment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
::windows::imp::interface_hierarchy!(XmlDocumentFragment, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlDocumentFragment {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlDocumentFragment {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlDocumentFragment {}
unsafe impl ::core::marker::Send for XmlDocumentFragment {}
unsafe impl ::core::marker::Sync for XmlDocumentFragment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentType(::windows::core::IUnknown);
impl XmlDocumentType {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Entities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Notations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
}
impl ::core::clone::Clone for XmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlDocumentType {
    const IID: ::windows::core::GUID = <IXmlDocumentType as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
::windows::imp::interface_hierarchy!(XmlDocumentType, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlDocumentType {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlDocumentType {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlDocumentType {}
unsafe impl ::core::marker::Send for XmlDocumentType {}
unsafe impl ::core::marker::Sync for XmlDocumentType {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDomImplementation(::windows::core::IUnknown);
impl XmlDomImplementation {
    pub fn HasFeature<P0>(&self, feature: &::windows::core::HSTRING, version: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasFeature)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(feature), version.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
}
impl ::core::clone::Clone for XmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlDomImplementation {
    const IID: ::windows::core::GUID = <IXmlDomImplementation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
::windows::imp::interface_hierarchy!(XmlDomImplementation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for XmlDomImplementation {}
unsafe impl ::core::marker::Sync for XmlDomImplementation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlElement(::windows::core::IUnknown);
impl XmlElement {
    pub fn TagName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TagName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetAttribute)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), &mut result__).from_abi(result__)
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
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).GetAttributeNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttributeNode(&self, newattribute: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).SetAttributeNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(newattribute), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNode(&self, attributenode: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).RemoveAttributeNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attributenode), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).GetElementsByTagName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAttributeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname)).ok() }
    }
    pub fn SetAttributeNodeNS(&self, newattribute: &XmlAttribute) -> ::windows::core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).SetAttributeNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(newattribute), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAttributeNodeNS<P0>(&self, namespaceuri: P0, localname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlAttribute>();
            (::windows::core::Interface::vtable(this).GetAttributeNodeNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlElement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
}
impl ::core::clone::Clone for XmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlElement {
    type Vtable = IXmlElement_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlElement {
    const IID: ::windows::core::GUID = <IXmlElement as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
::windows::imp::interface_hierarchy!(XmlElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlElement {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlElement {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlElement {}
unsafe impl ::core::marker::Send for XmlElement {}
unsafe impl ::core::marker::Sync for XmlElement {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlEntityReference(::windows::core::IUnknown);
impl XmlEntityReference {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
}
impl ::core::clone::Clone for XmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlEntityReference {
    const IID: ::windows::core::GUID = <IXmlEntityReference as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
::windows::imp::interface_hierarchy!(XmlEntityReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlEntityReference {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlEntityReference {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlEntityReference {}
unsafe impl ::core::marker::Send for XmlEntityReference {}
unsafe impl ::core::marker::Sync for XmlEntityReference {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlLoadSettings(::windows::core::IUnknown);
impl XmlLoadSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<XmlLoadSettings, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxElementDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxElementDepth)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxElementDepth)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ProhibitDtd)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProhibitDtd)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ResolveExternals)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResolveExternals)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ValidateOnParse)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValidateOnParse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ElementContentWhiteSpace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetElementContentWhiteSpace)(::windows::core::Interface::as_raw(this), value).ok() }
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
impl ::windows::core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
}
impl ::core::clone::Clone for XmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlLoadSettings {
    const IID: ::windows::core::GUID = <IXmlLoadSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
::windows::imp::interface_hierarchy!(XmlLoadSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for XmlLoadSettings {}
unsafe impl ::core::marker::Sync for XmlLoadSettings {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNamedNodeMap(::windows::core::IUnknown);
impl XmlNamedNodeMap {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).Item)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).GetNamedItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamedItem<P0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SetNamedItem)(::windows::core::Interface::as_raw(this), node.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveNamedItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).GetNamedItemNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveNamedItemNS)(::windows::core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamedItemNS<P0>(&self, node: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SetNamedItemNS)(::windows::core::Interface::as_raw(this), node.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
}
impl ::core::clone::Clone for XmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlNamedNodeMap {
    const IID: ::windows::core::GUID = <IXmlNamedNodeMap as ::windows::core::ComInterface>::IID;
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
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows::core::ComInterface::cast(self).ok())
    }
}
::windows::imp::interface_hierarchy!(XmlNamedNodeMap, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNamedNodeMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNamedNodeMap {}
unsafe impl ::core::marker::Send for XmlNamedNodeMap {}
unsafe impl ::core::marker::Sync for XmlNamedNodeMap {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNodeList(::windows::core::IUnknown);
impl XmlNodeList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IIterator<IXmlNode>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).Item)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
}
impl ::core::clone::Clone for XmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlNodeList {
    const IID: ::windows::core::GUID = <IXmlNodeList as ::windows::core::ComInterface>::IID;
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
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows::core::ComInterface::cast(self).ok())
    }
}
::windows::imp::interface_hierarchy!(XmlNodeList, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNodeList {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNodeList {}
unsafe impl ::core::marker::Send for XmlNodeList {}
unsafe impl ::core::marker::Sync for XmlNodeList {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlProcessingInstruction(::windows::core::IUnknown);
impl XmlProcessingInstruction {
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Target)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
}
impl ::core::clone::Clone for XmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlProcessingInstruction {
    const IID: ::windows::core::GUID = <IXmlProcessingInstruction as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
::windows::imp::interface_hierarchy!(XmlProcessingInstruction, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlNode> for XmlProcessingInstruction {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlProcessingInstruction {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlProcessingInstruction {}
unsafe impl ::core::marker::Send for XmlProcessingInstruction {}
unsafe impl ::core::marker::Sync for XmlProcessingInstruction {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlText(::windows::core::IUnknown);
impl XmlText {
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SubstringData)(::windows::core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AppendData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertData)(::windows::core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DeleteData)(::windows::core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceData)(::windows::core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NodeValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows::core::Result<NodeType> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NodeType>();
            (::windows::core::Interface::vtable(this).NodeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NodeName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ParentNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).ChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).FirstChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).LastChild)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).PreviousSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).NextSibling)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNamedNodeMap>();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasChildNodes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlDocument>();
            (::windows::core::Interface::vtable(this).OwnerDocument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).InsertBefore)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
        P1: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).ReplaceChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).RemoveChild)(::windows::core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).AppendChild)(::windows::core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).CloneNode)(::windows::core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Prefix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Normalize)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrefix)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<IXmlNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlNode>();
            (::windows::core::Interface::vtable(this).SelectSingleNodeNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows::core::HSTRING, namespaces: P0) -> ::windows::core::Result<XmlNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<XmlNodeList>();
            (::windows::core::Interface::vtable(this).SelectNodesNS)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetXml)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InnerText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInnerText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IXmlText>();
            (::windows::core::Interface::vtable(this).SplitText)(::windows::core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for XmlText {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
}
impl ::core::clone::Clone for XmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XmlText {
    type Vtable = IXmlText_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XmlText {
    const IID: ::windows::core::GUID = <IXmlText as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
::windows::imp::interface_hierarchy!(XmlText, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IXmlCharacterData> for XmlText {}
impl ::windows::core::CanTryInto<IXmlNode> for XmlText {}
impl ::windows::core::CanTryInto<IXmlNodeSelector> for XmlText {}
impl ::windows::core::CanTryInto<IXmlNodeSerializer> for XmlText {}
impl ::windows::core::CanTryInto<IXmlText> for XmlText {}
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
impl ::windows::core::TypeKind for NodeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NodeType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NodeType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

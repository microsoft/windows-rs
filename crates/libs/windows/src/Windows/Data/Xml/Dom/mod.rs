#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdEntity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
impl ::core::clone::Clone for IDtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDtdEntity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdNotation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
impl ::core::clone::Clone for IDtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDtdNotation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlAttribute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
impl ::core::clone::Clone for IXmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlAttribute {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlCDataSection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
impl ::core::clone::Clone for IXmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlCDataSection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlCharacterData(::windows_core::IUnknown);
impl IXmlCharacterData {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IXmlCharacterData, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for IXmlCharacterData {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for IXmlCharacterData {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for IXmlCharacterData {}
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
impl ::windows_core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
}
unsafe impl ::windows_core::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_Vtbl;
}
impl ::core::clone::Clone for IXmlCharacterData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlCharacterData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlComment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlComment {
    type Vtable = IXmlComment_Vtbl;
}
impl ::core::clone::Clone for IXmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlComment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
impl ::core::clone::Clone for IXmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows_core::HSTRING>, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentFragment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocumentFragment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentIO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocumentIO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LoadXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows_core::HSTRING>, loadsettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveToFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentIO2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocumentIO2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBufferWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBufferWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocumentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5543d254_d757_4b79_9539_232b18f50bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileWithSettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentType(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
impl ::core::clone::Clone for IXmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDocumentType {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDomImplementation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
impl ::core::clone::Clone for IXmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlDomImplementation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::HSTRING>, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlElement {
    type Vtable = IXmlElement_Vtbl;
}
impl ::core::clone::Clone for IXmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, attributevalue: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlEntityReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
impl ::core::clone::Clone for IXmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlEntityReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlLoadSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
impl ::core::clone::Clone for IXmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlLoadSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNamedNodeMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
impl ::core::clone::Clone for IXmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlNamedNodeMap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNode(::windows_core::IUnknown);
impl IXmlNode {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IXmlNode, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNodeSelector> for IXmlNode {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for IXmlNode {}
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
impl ::windows_core::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
}
unsafe impl ::windows_core::Interface for IXmlNode {
    type Vtable = IXmlNode_Vtbl;
}
impl ::core::clone::Clone for IXmlNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlNode {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows_core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LastChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HasChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Prefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNodeList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
}
impl ::core::clone::Clone for IXmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlNodeList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSelector(::windows_core::IUnknown);
impl IXmlNodeSelector {
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IXmlNodeSelector, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
}
unsafe impl ::windows_core::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_Vtbl;
}
impl ::core::clone::Clone for IXmlNodeSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlNodeSelector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlNodeSerializer(::windows_core::IUnknown);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IXmlNodeSerializer, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
}
unsafe impl ::windows_core::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_Vtbl;
}
impl ::core::clone::Clone for IXmlNodeSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlNodeSerializer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlProcessingInstruction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
impl ::core::clone::Clone for IXmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlProcessingInstruction {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct IXmlText(::windows_core::IUnknown);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IXmlText, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlCharacterData> for IXmlText {}
impl ::windows_core::CanTryInto<IXmlNode> for IXmlText {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for IXmlText {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for IXmlText {}
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
impl ::windows_core::RuntimeType for IXmlText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
}
unsafe impl ::windows_core::Interface for IXmlText {
    type Vtable = IXmlText_Vtbl;
}
impl ::core::clone::Clone for IXmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXmlText {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SplitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdEntity(::windows_core::IUnknown);
impl DtdEntity {
    pub fn PublicId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublicId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NotationName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotationName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
}
impl ::core::clone::Clone for DtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DtdEntity {
    const IID: ::windows_core::GUID = <IDtdEntity as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
::windows_core::imp::interface_hierarchy!(DtdEntity, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for DtdEntity {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for DtdEntity {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for DtdEntity {}
unsafe impl ::core::marker::Send for DtdEntity {}
unsafe impl ::core::marker::Sync for DtdEntity {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct DtdNotation(::windows_core::IUnknown);
impl DtdNotation {
    pub fn PublicId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublicId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
}
impl ::core::clone::Clone for DtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DtdNotation {
    const IID: ::windows_core::GUID = <IDtdNotation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
::windows_core::imp::interface_hierarchy!(DtdNotation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for DtdNotation {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for DtdNotation {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for DtdNotation {}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlAttribute(::windows_core::IUnknown);
impl XmlAttribute {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Specified(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Specified)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
}
impl ::core::clone::Clone for XmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlAttribute {
    const IID: ::windows_core::GUID = <IXmlAttribute as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
::windows_core::imp::interface_hierarchy!(XmlAttribute, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlAttribute {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlAttribute {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlAttribute {}
unsafe impl ::core::marker::Send for XmlAttribute {}
unsafe impl ::core::marker::Sync for XmlAttribute {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlCDataSection(::windows_core::IUnknown);
impl XmlCDataSection {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = &::windows_core::ComInterface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
}
impl ::core::clone::Clone for XmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlCDataSection {
    const IID: ::windows_core::GUID = <IXmlCDataSection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
::windows_core::imp::interface_hierarchy!(XmlCDataSection, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlCharacterData> for XmlCDataSection {}
impl ::windows_core::CanTryInto<IXmlNode> for XmlCDataSection {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlCDataSection {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlCDataSection {}
impl ::windows_core::CanTryInto<IXmlText> for XmlCDataSection {}
unsafe impl ::core::marker::Send for XmlCDataSection {}
unsafe impl ::core::marker::Sync for XmlCDataSection {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlComment(::windows_core::IUnknown);
impl XmlComment {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlComment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
}
impl ::core::clone::Clone for XmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlComment {
    type Vtable = IXmlComment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlComment {
    const IID: ::windows_core::GUID = <IXmlComment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
::windows_core::imp::interface_hierarchy!(XmlComment, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlCharacterData> for XmlComment {}
impl ::windows_core::CanTryInto<IXmlNode> for XmlComment {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlComment {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlComment {}
unsafe impl ::core::marker::Send for XmlComment {}
unsafe impl ::core::marker::Sync for XmlComment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocument(::windows_core::IUnknown);
impl XmlDocument {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XmlDocument, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Doctype(&self) -> ::windows_core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Doctype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Implementation(&self) -> ::windows_core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Implementation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentElement(&self) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentElement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateElement(&self, tagname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateElement)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateDocumentFragment(&self) -> ::windows_core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDocumentFragment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTextNode(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateComment(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateProcessingInstruction(&self, target: &::windows_core::HSTRING, data: &::windows_core::HSTRING) -> ::windows_core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateProcessingInstruction)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(target), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAttribute(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttribute)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateEntityReference(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateEntityReference)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetElementsByTagName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateCDataSection(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCDataSection)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlAttribute>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateElementNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlElement>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateElementNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementById(&self, elementid: &::windows_core::HSTRING) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetElementById)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(elementid), &mut result__).from_abi(result__)
        }
    }
    pub fn ImportNode<P0>(&self, node: P0, deep: bool) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportNode)(::windows_core::Interface::as_raw(this), node.try_into_param()?.abi(), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn LoadXml(&self, xml: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXml)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xml)).ok() }
    }
    pub fn LoadXmlWithSettings<P0>(&self, xml: &::windows_core::HSTRING, loadsettings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<XmlLoadSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xml), loadsettings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveToFileAsync<P0>(&self, file: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveToFileAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlFromBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBufferWithSettings<P0, P1>(&self, buffer: P0, loadsettings: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<XmlLoadSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlFromBufferWithSettings)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), loadsettings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriAsync<P0>(uri: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadFromUriWithSettingsAsync<P0, P1>(uri: P0, loadsettings: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<XmlLoadSettings>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromUriWithSettingsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), loadsettings.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileAsync<P0>(file: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileWithSettingsAsync<P0, P1>(file: P0, loadsettings: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::IStorageFile>,
        P1: ::windows_core::IntoParam<XmlLoadSettings>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileWithSettingsAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), loadsettings.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc(hidden)]
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
}
impl ::core::clone::Clone for XmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlDocument {
    const IID: ::windows_core::GUID = <IXmlDocument as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
::windows_core::imp::interface_hierarchy!(XmlDocument, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlDocument {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlDocument {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlDocument {}
unsafe impl ::core::marker::Send for XmlDocument {}
unsafe impl ::core::marker::Sync for XmlDocument {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentFragment(::windows_core::IUnknown);
impl XmlDocumentFragment {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
}
impl ::core::clone::Clone for XmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlDocumentFragment {
    const IID: ::windows_core::GUID = <IXmlDocumentFragment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
::windows_core::imp::interface_hierarchy!(XmlDocumentFragment, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlDocumentFragment {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlDocumentFragment {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlDocumentFragment {}
unsafe impl ::core::marker::Send for XmlDocumentFragment {}
unsafe impl ::core::marker::Sync for XmlDocumentFragment {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDocumentType(::windows_core::IUnknown);
impl XmlDocumentType {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Entities(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Notations(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Notations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
}
impl ::core::clone::Clone for XmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlDocumentType {
    const IID: ::windows_core::GUID = <IXmlDocumentType as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
::windows_core::imp::interface_hierarchy!(XmlDocumentType, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlDocumentType {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlDocumentType {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlDocumentType {}
unsafe impl ::core::marker::Send for XmlDocumentType {}
unsafe impl ::core::marker::Sync for XmlDocumentType {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlDomImplementation(::windows_core::IUnknown);
impl XmlDomImplementation {
    pub fn HasFeature<P0>(&self, feature: &::windows_core::HSTRING, version: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasFeature)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(feature), version.into_param().abi(), &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
}
impl ::core::clone::Clone for XmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlDomImplementation {
    const IID: ::windows_core::GUID = <IXmlDomImplementation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
::windows_core::imp::interface_hierarchy!(XmlDomImplementation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for XmlDomImplementation {}
unsafe impl ::core::marker::Sync for XmlDomImplementation {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlElement(::windows_core::IUnknown);
impl XmlElement {
    pub fn TagName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TagName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAttribute(&self, attributename: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttribute)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttribute(&self, attributename: &::windows_core::HSTRING, attributevalue: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttribute)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), ::core::mem::transmute_copy(attributevalue)).ok() }
    }
    pub fn RemoveAttribute(&self, attributename: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAttribute)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename)).ok() }
    }
    pub fn GetAttributeNode(&self, attributename: &::windows_core::HSTRING) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttributeNode<P0>(&self, newattribute: P0) -> ::windows_core::Result<XmlAttribute>
    where
        P0: ::windows_core::IntoParam<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributeNode)(::windows_core::Interface::as_raw(this), newattribute.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNode<P0>(&self, attributenode: P0) -> ::windows_core::Result<XmlAttribute>
    where
        P0: ::windows_core::IntoParam<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAttributeNode)(::windows_core::Interface::as_raw(this), attributenode.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetElementsByTagName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tagname), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(qualifiedname), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveAttributeNS<P0>(&self, namespaceuri: P0, localname: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname)).ok() }
    }
    pub fn SetAttributeNodeNS<P0>(&self, newattribute: P0) -> ::windows_core::Result<XmlAttribute>
    where
        P0: ::windows_core::IntoParam<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributeNodeNS)(::windows_core::Interface::as_raw(this), newattribute.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAttributeNodeNS<P0>(&self, namespaceuri: P0, localname: &::windows_core::HSTRING) -> ::windows_core::Result<XmlAttribute>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNodeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(localname), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
}
impl ::core::clone::Clone for XmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlElement {
    type Vtable = IXmlElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlElement {
    const IID: ::windows_core::GUID = <IXmlElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
::windows_core::imp::interface_hierarchy!(XmlElement, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlElement {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlElement {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlElement {}
unsafe impl ::core::marker::Send for XmlElement {}
unsafe impl ::core::marker::Sync for XmlElement {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlEntityReference(::windows_core::IUnknown);
impl XmlEntityReference {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
}
impl ::core::clone::Clone for XmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlEntityReference {
    const IID: ::windows_core::GUID = <IXmlEntityReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
::windows_core::imp::interface_hierarchy!(XmlEntityReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlEntityReference {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlEntityReference {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlEntityReference {}
unsafe impl ::core::marker::Send for XmlEntityReference {}
unsafe impl ::core::marker::Sync for XmlEntityReference {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlLoadSettings(::windows_core::IUnknown);
impl XmlLoadSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XmlLoadSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxElementDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxElementDepth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxElementDepth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProhibitDtd)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProhibitDtd)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolveExternals)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResolveExternals)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidateOnParse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValidateOnParse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementContentWhiteSpace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetElementContentWhiteSpace)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
}
impl ::core::clone::Clone for XmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlLoadSettings {
    const IID: ::windows_core::GUID = <IXmlLoadSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
::windows_core::imp::interface_hierarchy!(XmlLoadSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for XmlLoadSettings {}
unsafe impl ::core::marker::Sync for XmlLoadSettings {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNamedNodeMap(::windows_core::IUnknown);
impl XmlNamedNodeMap {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedItem(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamedItem<P0>(&self, node: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetNamedItem)(::windows_core::Interface::as_raw(this), node.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveNamedItem(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveNamedItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedItemNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveNamedItemNS<P0>(&self, namespaceuri: P0, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveNamedItemNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamedItemNS<P0>(&self, node: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetNamedItemNS)(::windows_core::Interface::as_raw(this), node.try_into_param()?.abi(), &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
}
impl ::core::clone::Clone for XmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlNamedNodeMap {
    const IID: ::windows_core::GUID = <IXmlNamedNodeMap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlNamedNodeMap {
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
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
::windows_core::imp::interface_hierarchy!(XmlNamedNodeMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNamedNodeMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNamedNodeMap {}
unsafe impl ::core::marker::Send for XmlNamedNodeMap {}
unsafe impl ::core::marker::Sync for XmlNamedNodeMap {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlNodeList(::windows_core::IUnknown);
impl XmlNodeList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
}
impl ::core::clone::Clone for XmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlNodeList {
    const IID: ::windows_core::GUID = <IXmlNodeList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlNodeList {
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
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
::windows_core::imp::interface_hierarchy!(XmlNodeList, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IXmlNode>> for XmlNodeList {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<IXmlNode>> for XmlNodeList {}
unsafe impl ::core::marker::Send for XmlNodeList {}
unsafe impl ::core::marker::Sync for XmlNodeList {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlProcessingInstruction(::windows_core::IUnknown);
impl XmlProcessingInstruction {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Target(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Target)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
}
impl ::core::clone::Clone for XmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlProcessingInstruction {
    const IID: ::windows_core::GUID = <IXmlProcessingInstruction as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
::windows_core::imp::interface_hierarchy!(XmlProcessingInstruction, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlNode> for XmlProcessingInstruction {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlProcessingInstruction {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlProcessingInstruction {}
unsafe impl ::core::marker::Send for XmlProcessingInstruction {}
unsafe impl ::core::marker::Sync for XmlProcessingInstruction {}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct XmlText(::windows_core::IUnknown);
impl XmlText {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, &mut result__).from_abi(result__)
        }
    }
    pub fn AppendData(&self, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
        P1: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), referencechild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::TryIntoParam<IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, &mut result__).from_abi(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodes(&self, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<IXmlNode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &::windows_core::HSTRING, namespaces: P0) -> ::windows_core::Result<XmlNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xpath), namespaces.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInnerText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for XmlText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
}
impl ::core::clone::Clone for XmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XmlText {
    type Vtable = IXmlText_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XmlText {
    const IID: ::windows_core::GUID = <IXmlText as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
::windows_core::imp::interface_hierarchy!(XmlText, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IXmlCharacterData> for XmlText {}
impl ::windows_core::CanTryInto<IXmlNode> for XmlText {}
impl ::windows_core::CanTryInto<IXmlNodeSelector> for XmlText {}
impl ::windows_core::CanTryInto<IXmlNodeSerializer> for XmlText {}
impl ::windows_core::CanTryInto<IXmlText> for XmlText {}
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
impl ::windows_core::TypeKind for NodeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NodeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NodeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

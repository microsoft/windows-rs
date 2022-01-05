#[cfg(feature = "implement_exclusive")]
pub trait IDtdEntityImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDtdNotationImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlAttributeImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Specified(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlCDataSectionImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl + IXmlTextImpl {}
pub trait IXmlCharacterDataImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlCommentImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Doctype(&self) -> ::windows::core::Result<XmlDocumentType>;
    fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation>;
    fn DocumentElement(&self) -> ::windows::core::Result<XmlElement>;
    fn CreateElement(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment>;
    fn CreateTextNode(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlText>;
    fn CreateComment(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlComment>;
    fn CreateProcessingInstruction(&self, target: &::windows::core::HSTRING, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlProcessingInstruction>;
    fn CreateAttribute(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn CreateEntityReference(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlEntityReference>;
    fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn CreateCDataSection(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlCDataSection>;
    fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn CreateElementNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn GetElementById(&self, elementid: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn ImportNode(&self, node: &::core::option::Option<IXmlNode>, deep: bool) -> ::windows::core::Result<IXmlNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentFragmentImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentIOImpl: Sized {
    fn LoadXml(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadXmlWithSettings(&self, xml: &::windows::core::HSTRING, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<()>;
    fn SaveToFileAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentIO2Impl: Sized {
    fn LoadXmlFromBuffer(&self, buffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LoadXmlFromBufferWithSettings(&self, buffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentStaticsImpl: Sized {
    fn LoadFromUriAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromUriWithSettingsAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromFileAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromFileWithSettingsAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentTypeImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
    fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDomImplementationImpl: Sized {
    fn HasFeature(&self, feature: &::windows::core::HSTRING, version: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlElementImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn TagName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAttribute(&self, attributename: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAttributeNode(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn SetAttributeNode(&self, newattribute: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn RemoveAttributeNode(&self, attributenode: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn SetAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetAttributeNodeNS(&self, newattribute: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn GetAttributeNodeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlEntityReferenceImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlLoadSettingsImpl: Sized {
    fn MaxElementDepth(&self) -> ::windows::core::Result<u32>;
    fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()>;
    fn ProhibitDtd(&self) -> ::windows::core::Result<bool>;
    fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()>;
    fn ResolveExternals(&self) -> ::windows::core::Result<bool>;
    fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()>;
    fn ValidateOnParse(&self) -> ::windows::core::Result<bool>;
    fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()>;
    fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool>;
    fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXmlNamedNodeMapImpl: Sized + IIterableImpl<IXmlNode> + IVectorViewImpl<IXmlNode> {
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode>;
    fn GetNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SetNamedItem(&self, node: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn RemoveNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn GetNamedItemNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn RemoveNamedItemNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SetNamedItemNS(&self, node: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
}
pub trait IXmlNodeImpl: Sized + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetNodeValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn NodeType(&self) -> ::windows::core::Result<NodeType>;
    fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentNode(&self) -> ::windows::core::Result<IXmlNode>;
    fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList>;
    fn FirstChild(&self) -> ::windows::core::Result<IXmlNode>;
    fn LastChild(&self) -> ::windows::core::Result<IXmlNode>;
    fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode>;
    fn NextSibling(&self) -> ::windows::core::Result<IXmlNode>;
    fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
    fn HasChildNodes(&self) -> ::windows::core::Result<bool>;
    fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument>;
    fn InsertBefore(&self, newchild: &::core::option::Option<IXmlNode>, referencechild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn ReplaceChild(&self, newchild: &::core::option::Option<IXmlNode>, referencechild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn RemoveChild(&self, childnode: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn AppendChild(&self, newchild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode>;
    fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Normalize(&self) -> ::windows::core::Result<()>;
    fn SetPrefix(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXmlNodeListImpl: Sized + IIterableImpl<IXmlNode> + IVectorViewImpl<IXmlNode> {
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode>;
}
pub trait IXmlNodeSelectorImpl: Sized {
    fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn SelectSingleNodeNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodesNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XmlNodeList>;
}
pub trait IXmlNodeSerializerImpl: Sized {
    fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlProcessingInstructionImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IXmlTextImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText>;
}

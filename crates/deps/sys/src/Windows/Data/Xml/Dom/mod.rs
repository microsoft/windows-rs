#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DtdEntity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DtdEntity {}
impl ::core::clone::Clone for DtdEntity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DtdNotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DtdNotation {}
impl ::core::clone::Clone for DtdNotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtdEntity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtdEntity {}
impl ::core::clone::Clone for IDtdEntity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtdNotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtdNotation {}
impl ::core::clone::Clone for IDtdNotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlAttribute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlAttribute {}
impl ::core::clone::Clone for IXmlAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlCDataSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlCDataSection {}
impl ::core::clone::Clone for IXmlCDataSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlCharacterData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlCharacterData {}
impl ::core::clone::Clone for IXmlCharacterData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlComment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlComment {}
impl ::core::clone::Clone for IXmlComment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocument {}
impl ::core::clone::Clone for IXmlDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocumentFragment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocumentFragment {}
impl ::core::clone::Clone for IXmlDocumentFragment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocumentIO(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocumentIO {}
impl ::core::clone::Clone for IXmlDocumentIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocumentIO2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocumentIO2 {}
impl ::core::clone::Clone for IXmlDocumentIO2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocumentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocumentStatics {}
impl ::core::clone::Clone for IXmlDocumentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDocumentType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDocumentType {}
impl ::core::clone::Clone for IXmlDocumentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlDomImplementation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlDomImplementation {}
impl ::core::clone::Clone for IXmlDomImplementation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlElement {}
impl ::core::clone::Clone for IXmlElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlEntityReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlEntityReference {}
impl ::core::clone::Clone for IXmlEntityReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlLoadSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlLoadSettings {}
impl ::core::clone::Clone for IXmlLoadSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlNamedNodeMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlNamedNodeMap {}
impl ::core::clone::Clone for IXmlNamedNodeMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlNode {}
impl ::core::clone::Clone for IXmlNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlNodeList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlNodeList {}
impl ::core::clone::Clone for IXmlNodeList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlNodeSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlNodeSelector {}
impl ::core::clone::Clone for IXmlNodeSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlNodeSerializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlNodeSerializer {}
impl ::core::clone::Clone for IXmlNodeSerializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlProcessingInstruction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlProcessingInstruction {}
impl ::core::clone::Clone for IXmlProcessingInstruction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlText {}
impl ::core::clone::Clone for IXmlText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct XmlAttribute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlAttribute {}
impl ::core::clone::Clone for XmlAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlCDataSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlCDataSection {}
impl ::core::clone::Clone for XmlCDataSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlComment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlComment {}
impl ::core::clone::Clone for XmlComment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlDocument {}
impl ::core::clone::Clone for XmlDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlDocumentFragment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlDocumentFragment {}
impl ::core::clone::Clone for XmlDocumentFragment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlDocumentType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlDocumentType {}
impl ::core::clone::Clone for XmlDocumentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlDomImplementation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlDomImplementation {}
impl ::core::clone::Clone for XmlDomImplementation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlElement {}
impl ::core::clone::Clone for XmlElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlEntityReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlEntityReference {}
impl ::core::clone::Clone for XmlEntityReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlLoadSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlLoadSettings {}
impl ::core::clone::Clone for XmlLoadSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlNamedNodeMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlNamedNodeMap {}
impl ::core::clone::Clone for XmlNamedNodeMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlNodeList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlNodeList {}
impl ::core::clone::Clone for XmlNodeList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlProcessingInstruction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlProcessingInstruction {}
impl ::core::clone::Clone for XmlProcessingInstruction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XmlText {}
impl ::core::clone::Clone for XmlText {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DtdEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DtdNotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtdEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtdNotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlCDataSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlCharacterData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlComment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocumentFragment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocumentIO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocumentIO2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocumentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDocumentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlDomImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlEntityReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlLoadSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlNamedNodeMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlNodeList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlNodeSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlNodeSerializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlProcessingInstruction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlText(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NodeType(i32);
#[repr(transparent)]
pub struct XmlAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlCDataSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlComment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlDocumentFragment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlDocumentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlDomImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlEntityReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlLoadSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlNamedNodeMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlNodeList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlProcessingInstruction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlText(pub *mut ::core::ffi::c_void);

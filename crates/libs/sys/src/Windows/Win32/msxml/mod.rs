pub const DOMDocument: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2933bf90_7b36_11d2_b20e_00c04f983e60);
pub const DOMFreeThreadedDocument: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2933bf91_7b36_11d2_b20e_00c04f983e60);
pub type DOMNodeType = i32;
pub const NODE_ATTRIBUTE: DOMNodeType = 2;
pub const NODE_CDATA_SECTION: DOMNodeType = 4;
pub const NODE_COMMENT: DOMNodeType = 8;
pub const NODE_DOCUMENT: DOMNodeType = 9;
pub const NODE_DOCUMENT_FRAGMENT: DOMNodeType = 11;
pub const NODE_DOCUMENT_TYPE: DOMNodeType = 10;
pub const NODE_ELEMENT: DOMNodeType = 1;
pub const NODE_ENTITY: DOMNodeType = 6;
pub const NODE_ENTITY_REFERENCE: DOMNodeType = 5;
pub const NODE_INVALID: DOMNodeType = 0;
pub const NODE_NOTATION: DOMNodeType = 12;
pub const NODE_PROCESSING_INSTRUCTION: DOMNodeType = 7;
pub const NODE_TEXT: DOMNodeType = 3;
pub const XMLDSOControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x550dda30_0541_11d2_9ca9_0060b0ec3d39);
pub const XMLDocument: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcfc399af_d876_11d0_9c10_00c04fc99c8e);
pub const XMLELEMTYPE_COMMENT: XMLELEM_TYPE = 2;
pub const XMLELEMTYPE_DOCUMENT: XMLELEM_TYPE = 3;
pub const XMLELEMTYPE_DTD: XMLELEM_TYPE = 4;
pub const XMLELEMTYPE_ELEMENT: XMLELEM_TYPE = 0;
pub const XMLELEMTYPE_OTHER: XMLELEM_TYPE = 6;
pub const XMLELEMTYPE_PI: XMLELEM_TYPE = 5;
pub const XMLELEMTYPE_TEXT: XMLELEM_TYPE = 1;
pub type XMLELEM_TYPE = i32;
pub const XMLHTTPRequest: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed8c108e_4349_11d2_91a4_00c04f7969e8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XML_ERROR {
    pub _nLine: u32,
    pub _pchBuf: windows_sys::core::BSTR,
    pub _cchBuf: u32,
    pub _ich: u32,
    pub _pszFound: windows_sys::core::BSTR,
    pub _pszExpected: windows_sys::core::BSTR,
    pub _reserved1: u32,
    pub _reserved2: u32,
}
impl Default for XML_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

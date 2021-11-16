#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlReader(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingName(pinputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, pwszencodingname: super::super::super::Foundation::PWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriter(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, nencodingcodepage: u32, ppoutput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlWriterOutputWithEncodingName(poutputstream: ::windows_sys::core::IUnknown, pmalloc: super::super::super::System::Com::IMalloc, pwszencodingname: super::super::super::Foundation::PWSTR, ppoutput: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
pub const DtdProcessing_Prohibit: i32 = 0i32;
pub const DtdProcessing_Parse: i32 = 1i32;
pub const _DtdProcessing_Last: i32 = 1i32;
#[repr(transparent)]
pub struct IXmlReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlReader {}
impl ::core::clone::Clone for IXmlReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlResolver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlResolver {}
impl ::core::clone::Clone for IXmlResolver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlWriter {}
impl ::core::clone::Clone for IXmlWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlWriterLite(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXmlWriterLite {}
impl ::core::clone::Clone for IXmlWriterLite {
    fn clone(&self) -> Self {
        *self
    }
}
pub const XmlConformanceLevel_Auto: i32 = 0i32;
pub const XmlConformanceLevel_Fragment: i32 = 1i32;
pub const XmlConformanceLevel_Document: i32 = 2i32;
pub const _XmlConformanceLevel_Last: i32 = 2i32;
pub const MX_E_MX: i32 = -1072894464i32;
pub const MX_E_INPUTEND: i32 = -1072894463i32;
pub const MX_E_ENCODING: i32 = -1072894462i32;
pub const MX_E_ENCODINGSWITCH: i32 = -1072894461i32;
pub const MX_E_ENCODINGSIGNATURE: i32 = -1072894460i32;
pub const WC_E_WC: i32 = -1072894432i32;
pub const WC_E_WHITESPACE: i32 = -1072894431i32;
pub const WC_E_SEMICOLON: i32 = -1072894430i32;
pub const WC_E_GREATERTHAN: i32 = -1072894429i32;
pub const WC_E_QUOTE: i32 = -1072894428i32;
pub const WC_E_EQUAL: i32 = -1072894427i32;
pub const WC_E_LESSTHAN: i32 = -1072894426i32;
pub const WC_E_HEXDIGIT: i32 = -1072894425i32;
pub const WC_E_DIGIT: i32 = -1072894424i32;
pub const WC_E_LEFTBRACKET: i32 = -1072894423i32;
pub const WC_E_LEFTPAREN: i32 = -1072894422i32;
pub const WC_E_XMLCHARACTER: i32 = -1072894421i32;
pub const WC_E_NAMECHARACTER: i32 = -1072894420i32;
pub const WC_E_SYNTAX: i32 = -1072894419i32;
pub const WC_E_CDSECT: i32 = -1072894418i32;
pub const WC_E_COMMENT: i32 = -1072894417i32;
pub const WC_E_CONDSECT: i32 = -1072894416i32;
pub const WC_E_DECLATTLIST: i32 = -1072894415i32;
pub const WC_E_DECLDOCTYPE: i32 = -1072894414i32;
pub const WC_E_DECLELEMENT: i32 = -1072894413i32;
pub const WC_E_DECLENTITY: i32 = -1072894412i32;
pub const WC_E_DECLNOTATION: i32 = -1072894411i32;
pub const WC_E_NDATA: i32 = -1072894410i32;
pub const WC_E_PUBLIC: i32 = -1072894409i32;
pub const WC_E_SYSTEM: i32 = -1072894408i32;
pub const WC_E_NAME: i32 = -1072894407i32;
pub const WC_E_ROOTELEMENT: i32 = -1072894406i32;
pub const WC_E_ELEMENTMATCH: i32 = -1072894405i32;
pub const WC_E_UNIQUEATTRIBUTE: i32 = -1072894404i32;
pub const WC_E_TEXTXMLDECL: i32 = -1072894403i32;
pub const WC_E_LEADINGXML: i32 = -1072894402i32;
pub const WC_E_TEXTDECL: i32 = -1072894401i32;
pub const WC_E_XMLDECL: i32 = -1072894400i32;
pub const WC_E_ENCNAME: i32 = -1072894399i32;
pub const WC_E_PUBLICID: i32 = -1072894398i32;
pub const WC_E_PESINTERNALSUBSET: i32 = -1072894397i32;
pub const WC_E_PESBETWEENDECLS: i32 = -1072894396i32;
pub const WC_E_NORECURSION: i32 = -1072894395i32;
pub const WC_E_ENTITYCONTENT: i32 = -1072894394i32;
pub const WC_E_UNDECLAREDENTITY: i32 = -1072894393i32;
pub const WC_E_PARSEDENTITY: i32 = -1072894392i32;
pub const WC_E_NOEXTERNALENTITYREF: i32 = -1072894391i32;
pub const WC_E_PI: i32 = -1072894390i32;
pub const WC_E_SYSTEMID: i32 = -1072894389i32;
pub const WC_E_QUESTIONMARK: i32 = -1072894388i32;
pub const WC_E_CDSECTEND: i32 = -1072894387i32;
pub const WC_E_MOREDATA: i32 = -1072894386i32;
pub const WC_E_DTDPROHIBITED: i32 = -1072894385i32;
pub const WC_E_INVALIDXMLSPACE: i32 = -1072894384i32;
pub const NC_E_NC: i32 = -1072894368i32;
pub const NC_E_QNAMECHARACTER: i32 = -1072894367i32;
pub const NC_E_QNAMECOLON: i32 = -1072894366i32;
pub const NC_E_NAMECOLON: i32 = -1072894365i32;
pub const NC_E_DECLAREDPREFIX: i32 = -1072894364i32;
pub const NC_E_UNDECLAREDPREFIX: i32 = -1072894363i32;
pub const NC_E_EMPTYURI: i32 = -1072894362i32;
pub const NC_E_XMLPREFIXRESERVED: i32 = -1072894361i32;
pub const NC_E_XMLNSPREFIXRESERVED: i32 = -1072894360i32;
pub const NC_E_XMLURIRESERVED: i32 = -1072894359i32;
pub const NC_E_XMLNSURIRESERVED: i32 = -1072894358i32;
pub const SC_E_SC: i32 = -1072894336i32;
pub const SC_E_MAXELEMENTDEPTH: i32 = -1072894335i32;
pub const SC_E_MAXENTITYEXPANSION: i32 = -1072894334i32;
pub const WR_E_WR: i32 = -1072894208i32;
pub const WR_E_NONWHITESPACE: i32 = -1072894207i32;
pub const WR_E_NSPREFIXDECLARED: i32 = -1072894206i32;
pub const WR_E_NSPREFIXWITHEMPTYNSURI: i32 = -1072894205i32;
pub const WR_E_DUPLICATEATTRIBUTE: i32 = -1072894204i32;
pub const WR_E_XMLNSPREFIXDECLARATION: i32 = -1072894203i32;
pub const WR_E_XMLPREFIXDECLARATION: i32 = -1072894202i32;
pub const WR_E_XMLURIDECLARATION: i32 = -1072894201i32;
pub const WR_E_XMLNSURIDECLARATION: i32 = -1072894200i32;
pub const WR_E_NAMESPACEUNDECLARED: i32 = -1072894199i32;
pub const WR_E_INVALIDXMLSPACE: i32 = -1072894198i32;
pub const WR_E_INVALIDACTION: i32 = -1072894197i32;
pub const WR_E_INVALIDSURROGATEPAIR: i32 = -1072894196i32;
pub const XML_E_INVALID_DECIMAL: i32 = -1072898019i32;
pub const XML_E_INVALID_HEXIDECIMAL: i32 = -1072898018i32;
pub const XML_E_INVALID_UNICODE: i32 = -1072898017i32;
pub const XML_E_INVALIDENCODING: i32 = -1072897938i32;
pub const XmlNodeType_None: i32 = 0i32;
pub const XmlNodeType_Element: i32 = 1i32;
pub const XmlNodeType_Attribute: i32 = 2i32;
pub const XmlNodeType_Text: i32 = 3i32;
pub const XmlNodeType_CDATA: i32 = 4i32;
pub const XmlNodeType_ProcessingInstruction: i32 = 7i32;
pub const XmlNodeType_Comment: i32 = 8i32;
pub const XmlNodeType_DocumentType: i32 = 10i32;
pub const XmlNodeType_Whitespace: i32 = 13i32;
pub const XmlNodeType_EndElement: i32 = 15i32;
pub const XmlNodeType_XmlDeclaration: i32 = 17i32;
pub const _XmlNodeType_Last: i32 = 17i32;
pub const XmlReadState_Initial: i32 = 0i32;
pub const XmlReadState_Interactive: i32 = 1i32;
pub const XmlReadState_Error: i32 = 2i32;
pub const XmlReadState_EndOfFile: i32 = 3i32;
pub const XmlReadState_Closed: i32 = 4i32;
pub const XmlReaderProperty_MultiLanguage: i32 = 0i32;
pub const XmlReaderProperty_ConformanceLevel: i32 = 1i32;
pub const XmlReaderProperty_RandomAccess: i32 = 2i32;
pub const XmlReaderProperty_XmlResolver: i32 = 3i32;
pub const XmlReaderProperty_DtdProcessing: i32 = 4i32;
pub const XmlReaderProperty_ReadState: i32 = 5i32;
pub const XmlReaderProperty_MaxElementDepth: i32 = 6i32;
pub const XmlReaderProperty_MaxEntityExpansion: i32 = 7i32;
pub const _XmlReaderProperty_Last: i32 = 7i32;
pub const XmlStandalone_Omit: i32 = 0i32;
pub const XmlStandalone_Yes: i32 = 1i32;
pub const XmlStandalone_No: i32 = 2i32;
pub const _XmlStandalone_Last: i32 = 2i32;
pub const XmlWriterProperty_MultiLanguage: i32 = 0i32;
pub const XmlWriterProperty_Indent: i32 = 1i32;
pub const XmlWriterProperty_ByteOrderMark: i32 = 2i32;
pub const XmlWriterProperty_OmitXmlDeclaration: i32 = 3i32;
pub const XmlWriterProperty_ConformanceLevel: i32 = 4i32;
pub const XmlWriterProperty_CompactEmptyElement: i32 = 5i32;
pub const _XmlWriterProperty_Last: i32 = 5i32;
pub const _IID_IXmlReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597121, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlResolver: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597122, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597128, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };

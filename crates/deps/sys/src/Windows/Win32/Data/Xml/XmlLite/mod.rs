#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct DtdProcessing(pub i32);
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::core::marker::Copy for DtdProcessing {}
impl ::core::clone::Clone for DtdProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXmlReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlResolver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXmlWriterLite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XmlConformanceLevel(pub i32);
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::core::marker::Copy for XmlConformanceLevel {}
impl ::core::clone::Clone for XmlConformanceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlError(pub i32);
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
impl ::core::marker::Copy for XmlError {}
impl ::core::clone::Clone for XmlError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlNodeType(pub i32);
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
impl ::core::marker::Copy for XmlNodeType {}
impl ::core::clone::Clone for XmlNodeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlReadState(pub i32);
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::core::marker::Copy for XmlReadState {}
impl ::core::clone::Clone for XmlReadState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlReaderProperty(pub i32);
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
impl ::core::marker::Copy for XmlReaderProperty {}
impl ::core::clone::Clone for XmlReaderProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlStandalone(pub i32);
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::core::marker::Copy for XmlStandalone {}
impl ::core::clone::Clone for XmlStandalone {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XmlWriterProperty(pub i32);
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::core::marker::Copy for XmlWriterProperty {}
impl ::core::clone::Clone for XmlWriterProperty {
    fn clone(&self) -> Self {
        *self
    }
}
pub const _IID_IXmlReader: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597121, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlResolver: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597122, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlWriter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1920597128, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };

#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AbortDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DeviceCapabilitiesA(pdevice: super::super::Foundation::PSTR, pport: super::super::Foundation::PSTR, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DeviceCapabilitiesW(pdevice: super::super::Foundation::PWSTR, pport: super::super::Foundation::PWSTR, fwcapability: DEVICE_CAPABILITIES, poutput: super::super::Foundation::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EndDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EndPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn Escape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjin: i32, pvin: super::super::Foundation::PSTR, pvout: *mut ::core::ffi::c_void) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ExtEscape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjinput: i32, lpindata: super::super::Foundation::PSTR, cjoutput: i32, lpoutdata: super::super::Foundation::PSTR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintWindow(hwnd: super::super::Foundation::HWND, hdcblt: super::super::Graphics::Gdi::HDC, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetAbortProc(hdc: super::super::Graphics::Gdi::HDC, proc: ABORTPROC) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn StartDocA(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn StartDocW(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOW) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn StartPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
}
pub struct ABORTPROC(i32);
pub struct DEVICE_CAPABILITIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DRAWPATRECT(i32);
pub struct HPTPROVIDER(i32);
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMCanvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMColorProfileResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMColorProfileResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMCoreProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDashCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDictionary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDocumentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDocumentSequence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMDocumentStructureResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMFontResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMFontResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGeometryFigure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGeometryFigureCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGlyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGlyphsEditor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGradientStop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMGradientStopCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMImageBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMImageResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMImageResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMLinearGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMMatrixTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMNameCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMObjectFactory1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPackage1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPackageTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPackageWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPackageWriter3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPage1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPageReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPageReferenceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPartResources(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPartUriCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMPrintTicketResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMRadialGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMShareable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMSolidColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMStoryFragmentsResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMThumbnailGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMTileBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMVisualBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsOMVisualCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureBlockCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSignatureRequestCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsSigningOptions(pub *mut ::core::ffi::c_void);
pub struct PRINT_WINDOW_FLAGS(i32);
pub struct PSFEATURE_CUSTPAPER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSFEATURE_OUTPUT(i32);
pub struct PSINJECTDATA(i32);
pub struct PSINJECT_POINT(i32);
pub struct XPS_COLOR(i32);
pub struct XPS_COLOR_INTERPOLATION(i32);
pub struct XPS_COLOR_TYPE(i32);
pub struct XPS_DASH(i32);
pub struct XPS_DASH_CAP(i32);
pub struct XPS_DOCUMENT_TYPE(i32);
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108159i32 as _);
pub const XPS_E_ALREADY_OWNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108413i32 as _);
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108407i32 as _);
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108409i32 as _);
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108408i32 as _);
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108923i32 as _);
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108922i32 as _);
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108410i32 as _);
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108671i32 as _);
pub const XPS_E_DUPLICATE_NAMES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109175i32 as _);
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109184i32 as _);
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108416i32 as _);
pub const XPS_E_INVALID_BLEED_BOX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109692i32 as _);
pub const XPS_E_INVALID_CONTENT_BOX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109685i32 as _);
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109682i32 as _);
pub const XPS_E_INVALID_FLOAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109689i32 as _);
pub const XPS_E_INVALID_FONT_URI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109686i32 as _);
pub const XPS_E_INVALID_LANGUAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109696i32 as _);
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109690i32 as _);
pub const XPS_E_INVALID_MARKUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109684i32 as _);
pub const XPS_E_INVALID_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109695i32 as _);
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108158i32 as _);
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108160i32 as _);
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109681i32 as _);
pub const XPS_E_INVALID_PAGE_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109693i32 as _);
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109694i32 as _);
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108789i32 as _);
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109691i32 as _);
pub const XPS_E_INVALID_XML_ENCODING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109683i32 as _);
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108924i32 as _);
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108925i32 as _);
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108926i32 as _);
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108791i32 as _);
pub const XPS_E_MISSING_COLORPROFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109436i32 as _);
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109422i32 as _);
pub const XPS_E_MISSING_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109431i32 as _);
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109432i32 as _);
pub const XPS_E_MISSING_FONTURI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109433i32 as _);
pub const XPS_E_MISSING_GLYPHS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109438i32 as _);
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109426i32 as _);
pub const XPS_E_MISSING_LOOKUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109439i32 as _);
pub const XPS_E_MISSING_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109440i32 as _);
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109428i32 as _);
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109427i32 as _);
pub const XPS_E_MISSING_PART_REFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109424i32 as _);
pub const XPS_E_MISSING_PART_STREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109421i32 as _);
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109430i32 as _);
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109429i32 as _);
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109435i32 as _);
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109425i32 as _);
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109434i32 as _);
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109423i32 as _);
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109437i32 as _);
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109182i32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109178i32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109177i32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109179i32 as _);
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109176i32 as _);
pub const XPS_E_MULTIPLE_RESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109183i32 as _);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109180i32 as _);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109181i32 as _);
pub const XPS_E_NEGATIVE_FLOAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108918i32 as _);
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108670i32 as _);
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108405i32 as _);
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108414i32 as _);
pub const XPS_E_OBJECT_DETACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108790i32 as _);
pub const XPS_E_ODD_BIDILEVEL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108921i32 as _);
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108920i32 as _);
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108793i32 as _);
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108794i32 as _);
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108404i32 as _);
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108406i32 as _);
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108412i32 as _);
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108919i32 as _);
pub const XPS_E_SIGNATUREID_DUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108792i32 as _);
pub const XPS_E_SIGREQUESTID_DUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108795i32 as _);
pub const XPS_E_STRING_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108928i32 as _);
pub const XPS_E_TOO_MANY_INDICES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108927i32 as _);
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109420i32 as _);
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108411i32 as _);
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109688i32 as _);
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109680i32 as _);
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142109679i32 as _);
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108415i32 as _);
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2142108672i32 as _);
pub struct XPS_FILL_RULE(i32);
pub struct XPS_FONT_EMBEDDING(i32);
pub struct XPS_GLYPH_INDEX(i32);
pub struct XPS_GLYPH_MAPPING(i32);
pub struct XPS_IMAGE_TYPE(i32);
pub struct XPS_INTERLEAVING(i32);
pub struct XPS_LINE_CAP(i32);
pub struct XPS_LINE_JOIN(i32);
pub struct XPS_MATRIX(i32);
pub struct XPS_OBJECT_TYPE(i32);
pub struct XPS_POINT(i32);
pub struct XPS_RECT(i32);
pub struct XPS_SEGMENT_STROKE_PATTERN(i32);
pub struct XPS_SEGMENT_TYPE(i32);
pub struct XPS_SIGNATURE_STATUS(i32);
pub struct XPS_SIGN_FLAGS(i32);
pub struct XPS_SIGN_POLICY(i32);
pub struct XPS_SIZE(i32);
pub struct XPS_SPREAD_METHOD(i32);
pub struct XPS_STYLE_SIMULATION(i32);
pub struct XPS_THUMBNAIL_SIZE(i32);
pub struct XPS_TILE_MODE(i32);
pub struct XpsOMObjectFactory(i32);
pub struct XpsOMThumbnailGenerator(i32);
pub struct XpsSignatureManager(i32);

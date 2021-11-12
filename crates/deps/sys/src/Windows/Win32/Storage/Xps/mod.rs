#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL;
#[repr(transparent)]
pub struct DEVICE_CAPABILITIES(pub u32);
pub const DC_BINNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(12u32);
pub const DC_BINS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(6u32);
pub const DC_COLLATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(22u32);
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(32u32);
pub const DC_COPIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(18u32);
pub const DC_DRIVER: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(11u32);
pub const DC_DUPLEX: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(7u32);
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(13u32);
pub const DC_EXTRA: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(9u32);
pub const DC_FIELDS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(1u32);
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(14u32);
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(5u32);
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(29u32);
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(34u32);
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(35u32);
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(4u32);
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(17u32);
pub const DC_NUP: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(33u32);
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(16u32);
pub const DC_PAPERS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(2u32);
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(3u32);
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(25u32);
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(28u32);
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(26u32);
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(31u32);
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(27u32);
pub const DC_SIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(8u32);
pub const DC_STAPLE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(30u32);
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(15u32);
pub const DC_VERSION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(10u32);
impl ::core::marker::Copy for DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PSTR,
    pub lpszOutput: super::super::Foundation::PSTR,
    pub lpszDatatype: super::super::Foundation::PSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOCINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: super::super::Foundation::PWSTR,
    pub lpszOutput: super::super::Foundation::PWSTR,
    pub lpszDatatype: super::super::Foundation::PWSTR,
    pub fwType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOCINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRAWPATRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HPTPROVIDER(pub isize);
impl ::core::marker::Copy for HPTPROVIDER {}
impl ::core::clone::Clone for HPTPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsDocumentPackageTarget {}
impl ::core::clone::Clone for IXpsDocumentPackageTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget3D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsDocumentPackageTarget3D {}
impl ::core::clone::Clone for IXpsDocumentPackageTarget3D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMBrush {}
impl ::core::clone::Clone for IXpsOMBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMCanvas(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMCanvas {}
impl ::core::clone::Clone for IXpsOMCanvas {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMColorProfileResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMColorProfileResource {}
impl ::core::clone::Clone for IXpsOMColorProfileResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMColorProfileResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMColorProfileResourceCollection {}
impl ::core::clone::Clone for IXpsOMColorProfileResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMCoreProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMCoreProperties {}
impl ::core::clone::Clone for IXpsOMCoreProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDashCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDashCollection {}
impl ::core::clone::Clone for IXpsOMDashCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDictionary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDictionary {}
impl ::core::clone::Clone for IXpsOMDictionary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDocument {}
impl ::core::clone::Clone for IXpsOMDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDocumentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDocumentCollection {}
impl ::core::clone::Clone for IXpsOMDocumentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDocumentSequence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDocumentSequence {}
impl ::core::clone::Clone for IXpsOMDocumentSequence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMDocumentStructureResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMDocumentStructureResource {}
impl ::core::clone::Clone for IXpsOMDocumentStructureResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMFontResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMFontResource {}
impl ::core::clone::Clone for IXpsOMFontResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMFontResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMFontResourceCollection {}
impl ::core::clone::Clone for IXpsOMFontResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGeometry {}
impl ::core::clone::Clone for IXpsOMGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGeometryFigure(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGeometryFigure {}
impl ::core::clone::Clone for IXpsOMGeometryFigure {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGeometryFigureCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGeometryFigureCollection {}
impl ::core::clone::Clone for IXpsOMGeometryFigureCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGlyphs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGlyphs {}
impl ::core::clone::Clone for IXpsOMGlyphs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGlyphsEditor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGlyphsEditor {}
impl ::core::clone::Clone for IXpsOMGlyphsEditor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGradientBrush {}
impl ::core::clone::Clone for IXpsOMGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGradientStop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGradientStop {}
impl ::core::clone::Clone for IXpsOMGradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMGradientStopCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMGradientStopCollection {}
impl ::core::clone::Clone for IXpsOMGradientStopCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMImageBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMImageBrush {}
impl ::core::clone::Clone for IXpsOMImageBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMImageResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMImageResource {}
impl ::core::clone::Clone for IXpsOMImageResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMImageResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMImageResourceCollection {}
impl ::core::clone::Clone for IXpsOMImageResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMLinearGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMLinearGradientBrush {}
impl ::core::clone::Clone for IXpsOMLinearGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMMatrixTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMMatrixTransform {}
impl ::core::clone::Clone for IXpsOMMatrixTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMNameCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMNameCollection {}
impl ::core::clone::Clone for IXpsOMNameCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMObjectFactory {}
impl ::core::clone::Clone for IXpsOMObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMObjectFactory1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMObjectFactory1 {}
impl ::core::clone::Clone for IXpsOMObjectFactory1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPackage {}
impl ::core::clone::Clone for IXpsOMPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPackage1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPackage1 {}
impl ::core::clone::Clone for IXpsOMPackage1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPackageTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPackageTarget {}
impl ::core::clone::Clone for IXpsOMPackageTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPackageWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPackageWriter {}
impl ::core::clone::Clone for IXpsOMPackageWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPackageWriter3D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPackageWriter3D {}
impl ::core::clone::Clone for IXpsOMPackageWriter3D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPage {}
impl ::core::clone::Clone for IXpsOMPage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPage1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPage1 {}
impl ::core::clone::Clone for IXpsOMPage1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPageReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPageReference {}
impl ::core::clone::Clone for IXpsOMPageReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPageReferenceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPageReferenceCollection {}
impl ::core::clone::Clone for IXpsOMPageReferenceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPart(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPart {}
impl ::core::clone::Clone for IXpsOMPart {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPartResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPartResources {}
impl ::core::clone::Clone for IXpsOMPartResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPartUriCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPartUriCollection {}
impl ::core::clone::Clone for IXpsOMPartUriCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPath {}
impl ::core::clone::Clone for IXpsOMPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMPrintTicketResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMPrintTicketResource {}
impl ::core::clone::Clone for IXpsOMPrintTicketResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMRadialGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMRadialGradientBrush {}
impl ::core::clone::Clone for IXpsOMRadialGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMRemoteDictionaryResource {}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMRemoteDictionaryResource1 {}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMRemoteDictionaryResourceCollection {}
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMResource {}
impl ::core::clone::Clone for IXpsOMResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMShareable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMShareable {}
impl ::core::clone::Clone for IXpsOMShareable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMSignatureBlockResource {}
impl ::core::clone::Clone for IXpsOMSignatureBlockResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMSignatureBlockResourceCollection {}
impl ::core::clone::Clone for IXpsOMSignatureBlockResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMSolidColorBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMSolidColorBrush {}
impl ::core::clone::Clone for IXpsOMSolidColorBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMStoryFragmentsResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMStoryFragmentsResource {}
impl ::core::clone::Clone for IXpsOMStoryFragmentsResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMThumbnailGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMThumbnailGenerator {}
impl ::core::clone::Clone for IXpsOMThumbnailGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMTileBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMTileBrush {}
impl ::core::clone::Clone for IXpsOMTileBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMVisual {}
impl ::core::clone::Clone for IXpsOMVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMVisualBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMVisualBrush {}
impl ::core::clone::Clone for IXpsOMVisualBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsOMVisualCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsOMVisualCollection {}
impl ::core::clone::Clone for IXpsOMVisualCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignature {}
impl ::core::clone::Clone for IXpsSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureBlock {}
impl ::core::clone::Clone for IXpsSignatureBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureBlockCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureBlockCollection {}
impl ::core::clone::Clone for IXpsSignatureBlockCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureCollection {}
impl ::core::clone::Clone for IXpsSignatureCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureManager {}
impl ::core::clone::Clone for IXpsSignatureManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureRequest {}
impl ::core::clone::Clone for IXpsSignatureRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSignatureRequestCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSignatureRequestCollection {}
impl ::core::clone::Clone for IXpsSignatureRequestCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXpsSigningOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXpsSigningOptions {}
impl ::core::clone::Clone for IXpsSigningOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
impl ::core::marker::Copy for PRINT_WINDOW_FLAGS {}
impl ::core::clone::Clone for PRINT_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl ::core::marker::Copy for PSFEATURE_CUSTPAPER {}
impl ::core::clone::Clone for PSFEATURE_CUSTPAPER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSFEATURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl ::core::marker::Copy for PSINJECTDATA {}
impl ::core::clone::Clone for PSINJECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PSINJECT_POINT(pub u16);
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
impl ::core::marker::Copy for PSINJECT_POINT {}
impl ::core::clone::Clone for PSINJECT_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl ::core::marker::Copy for XPS_COLOR {}
impl ::core::clone::Clone for XPS_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl ::core::marker::Copy for XPS_COLOR_0 {}
impl ::core::clone::Clone for XPS_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl ::core::marker::Copy for XPS_COLOR_0_0 {}
impl ::core::clone::Clone for XPS_COLOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl ::core::marker::Copy for XPS_COLOR_0_1 {}
impl ::core::clone::Clone for XPS_COLOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl ::core::marker::Copy for XPS_COLOR_0_2 {}
impl ::core::clone::Clone for XPS_COLOR_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
impl ::core::marker::Copy for XPS_COLOR_INTERPOLATION {}
impl ::core::clone::Clone for XPS_COLOR_INTERPOLATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_COLOR_TYPE(pub i32);
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
impl ::core::marker::Copy for XPS_COLOR_TYPE {}
impl ::core::clone::Clone for XPS_COLOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl ::core::marker::Copy for XPS_DASH {}
impl ::core::clone::Clone for XPS_DASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_DASH_CAP(pub i32);
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
impl ::core::marker::Copy for XPS_DASH_CAP {}
impl ::core::clone::Clone for XPS_DASH_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
impl ::core::marker::Copy for XPS_DOCUMENT_TYPE {}
impl ::core::clone::Clone for XPS_DOCUMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct XPS_FILL_RULE(pub i32);
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
impl ::core::marker::Copy for XPS_FILL_RULE {}
impl ::core::clone::Clone for XPS_FILL_RULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_FONT_EMBEDDING(pub i32);
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
impl ::core::marker::Copy for XPS_FONT_EMBEDDING {}
impl ::core::clone::Clone for XPS_FONT_EMBEDDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl ::core::marker::Copy for XPS_GLYPH_INDEX {}
impl ::core::clone::Clone for XPS_GLYPH_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl ::core::marker::Copy for XPS_GLYPH_MAPPING {}
impl ::core::clone::Clone for XPS_GLYPH_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_IMAGE_TYPE(pub i32);
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
impl ::core::marker::Copy for XPS_IMAGE_TYPE {}
impl ::core::clone::Clone for XPS_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_INTERLEAVING(pub i32);
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
impl ::core::marker::Copy for XPS_INTERLEAVING {}
impl ::core::clone::Clone for XPS_INTERLEAVING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_LINE_CAP(pub i32);
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
impl ::core::marker::Copy for XPS_LINE_CAP {}
impl ::core::clone::Clone for XPS_LINE_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_LINE_JOIN(pub i32);
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
impl ::core::marker::Copy for XPS_LINE_JOIN {}
impl ::core::clone::Clone for XPS_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl ::core::marker::Copy for XPS_MATRIX {}
impl ::core::clone::Clone for XPS_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_OBJECT_TYPE(pub i32);
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
impl ::core::marker::Copy for XPS_OBJECT_TYPE {}
impl ::core::clone::Clone for XPS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for XPS_POINT {}
impl ::core::clone::Clone for XPS_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_RECT {}
impl ::core::clone::Clone for XPS_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
impl ::core::marker::Copy for XPS_SEGMENT_STROKE_PATTERN {}
impl ::core::clone::Clone for XPS_SEGMENT_STROKE_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SEGMENT_TYPE(pub i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
impl ::core::marker::Copy for XPS_SEGMENT_TYPE {}
impl ::core::clone::Clone for XPS_SEGMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
impl ::core::marker::Copy for XPS_SIGNATURE_STATUS {}
impl ::core::clone::Clone for XPS_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SIGN_FLAGS(pub i32);
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
impl ::core::marker::Copy for XPS_SIGN_FLAGS {}
impl ::core::clone::Clone for XPS_SIGN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SIGN_POLICY(pub i32);
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
impl ::core::marker::Copy for XPS_SIGN_POLICY {}
impl ::core::clone::Clone for XPS_SIGN_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_SIZE {}
impl ::core::clone::Clone for XPS_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_SPREAD_METHOD(pub i32);
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
impl ::core::marker::Copy for XPS_SPREAD_METHOD {}
impl ::core::clone::Clone for XPS_SPREAD_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_STYLE_SIMULATION(pub i32);
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
impl ::core::marker::Copy for XPS_STYLE_SIMULATION {}
impl ::core::clone::Clone for XPS_STYLE_SIMULATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
impl ::core::marker::Copy for XPS_THUMBNAIL_SIZE {}
impl ::core::clone::Clone for XPS_THUMBNAIL_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XPS_TILE_MODE(pub i32);
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
impl ::core::marker::Copy for XPS_TILE_MODE {}
impl ::core::clone::Clone for XPS_TILE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const XpsOMObjectFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3916747373,
    data2: 15771,
    data3: 19783,
    data4: [136, 204, 56, 114, 242, 220, 53, 133],
};
pub const XpsOMThumbnailGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2118788066, data2: 47465, data3: 18273, data4: [190, 53, 26, 140, 237, 88, 227, 35] };
pub const XpsSignatureManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2965648160, data2: 8981, data3: 17570, data4: [183, 10, 9, 67, 161, 64, 168, 238] };

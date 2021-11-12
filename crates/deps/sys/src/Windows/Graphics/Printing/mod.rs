#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Graphics_Printing_OptionDetails")]
pub mod OptionDetails;
#[cfg(feature = "Graphics_Printing_PrintSupport")]
pub mod PrintSupport;
#[cfg(feature = "Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[cfg(feature = "Graphics_Printing_Workflow")]
pub mod Workflow;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintDocumentSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintDocumentSource {}
impl ::core::clone::Clone for IPrintDocumentSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintManager {}
impl ::core::clone::Clone for IPrintManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintManagerStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintManagerStatic {}
impl ::core::clone::Clone for IPrintManagerStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintManagerStatic2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintManagerStatic2 {}
impl ::core::clone::Clone for IPrintManagerStatic2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintPageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintPageInfo {}
impl ::core::clone::Clone for IPrintPageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintPageRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintPageRange {}
impl ::core::clone::Clone for IPrintPageRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintPageRangeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintPageRangeFactory {}
impl ::core::clone::Clone for IPrintPageRangeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintPageRangeOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintPageRangeOptions {}
impl ::core::clone::Clone for IPrintPageRangeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTask {}
impl ::core::clone::Clone for IPrintTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTask2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTask2 {}
impl ::core::clone::Clone for IPrintTask2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskCompletedEventArgs {}
impl ::core::clone::Clone for IPrintTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskOptions {}
impl ::core::clone::Clone for IPrintTaskOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskOptions2 {}
impl ::core::clone::Clone for IPrintTaskOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskOptionsCore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskOptionsCore {}
impl ::core::clone::Clone for IPrintTaskOptionsCore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskOptionsCoreProperties {}
impl ::core::clone::Clone for IPrintTaskOptionsCoreProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreUIConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskOptionsCoreUIConfiguration {}
impl ::core::clone::Clone for IPrintTaskOptionsCoreUIConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskProgressingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskProgressingEventArgs {}
impl ::core::clone::Clone for IPrintTaskProgressingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskRequest {}
impl ::core::clone::Clone for IPrintTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskRequestedDeferral {}
impl ::core::clone::Clone for IPrintTaskRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskRequestedEventArgs {}
impl ::core::clone::Clone for IPrintTaskRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskSourceRequestedArgs {}
impl ::core::clone::Clone for IPrintTaskSourceRequestedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskSourceRequestedDeferral {}
impl ::core::clone::Clone for IPrintTaskSourceRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskTargetDeviceSupport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskTargetDeviceSupport {}
impl ::core::clone::Clone for IPrintTaskTargetDeviceSupport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardPrintTaskOptionsStatic {}
impl ::core::clone::Clone for IStandardPrintTaskOptionsStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardPrintTaskOptionsStatic2 {}
impl ::core::clone::Clone for IStandardPrintTaskOptionsStatic2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardPrintTaskOptionsStatic3 {}
impl ::core::clone::Clone for IStandardPrintTaskOptionsStatic3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const Bale: Self = Self(4i32);
    pub const BindBottom: Self = Self(5i32);
    pub const BindLeft: Self = Self(6i32);
    pub const BindRight: Self = Self(7i32);
    pub const BindTop: Self = Self(8i32);
    pub const Booklet: Self = Self(9i32);
    pub const EdgeStitchBottom: Self = Self(10i32);
    pub const EdgeStitchLeft: Self = Self(11i32);
    pub const EdgeStitchRight: Self = Self(12i32);
    pub const EdgeStitchTop: Self = Self(13i32);
    pub const Fold: Self = Self(14i32);
    pub const JogOffset: Self = Self(15i32);
    pub const Trim: Self = Self(16i32);
}
impl ::core::marker::Copy for PrintBinding {}
impl ::core::clone::Clone for PrintBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Bordered: Self = Self(3i32);
    pub const Borderless: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintBordering {}
impl ::core::clone::Clone for PrintBordering {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Collated: Self = Self(3i32);
    pub const Uncollated: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintCollation {}
impl ::core::clone::Clone for PrintCollation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Color: Self = Self(3i32);
    pub const Grayscale: Self = Self(4i32);
    pub const Monochrome: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintColorMode {}
impl ::core::clone::Clone for PrintColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const OneSided: Self = Self(3i32);
    pub const TwoSidedShortEdge: Self = Self(4i32);
    pub const TwoSidedLongEdge: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintDuplex {}
impl ::core::clone::Clone for PrintDuplex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const LeftEdge: Self = Self(4i32);
    pub const RightEdge: Self = Self(5i32);
    pub const TopEdge: Self = Self(6i32);
    pub const BottomEdge: Self = Self(7i32);
}
impl ::core::marker::Copy for PrintHolePunch {}
impl ::core::clone::Clone for PrintHolePunch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintManager {}
impl ::core::clone::Clone for PrintManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const BusinessCard: Self = Self(3i32);
    pub const CreditCard: Self = Self(4i32);
    pub const IsoA0: Self = Self(5i32);
    pub const IsoA1: Self = Self(6i32);
    pub const IsoA10: Self = Self(7i32);
    pub const IsoA2: Self = Self(8i32);
    pub const IsoA3: Self = Self(9i32);
    pub const IsoA3Extra: Self = Self(10i32);
    pub const IsoA3Rotated: Self = Self(11i32);
    pub const IsoA4: Self = Self(12i32);
    pub const IsoA4Extra: Self = Self(13i32);
    pub const IsoA4Rotated: Self = Self(14i32);
    pub const IsoA5: Self = Self(15i32);
    pub const IsoA5Extra: Self = Self(16i32);
    pub const IsoA5Rotated: Self = Self(17i32);
    pub const IsoA6: Self = Self(18i32);
    pub const IsoA6Rotated: Self = Self(19i32);
    pub const IsoA7: Self = Self(20i32);
    pub const IsoA8: Self = Self(21i32);
    pub const IsoA9: Self = Self(22i32);
    pub const IsoB0: Self = Self(23i32);
    pub const IsoB1: Self = Self(24i32);
    pub const IsoB10: Self = Self(25i32);
    pub const IsoB2: Self = Self(26i32);
    pub const IsoB3: Self = Self(27i32);
    pub const IsoB4: Self = Self(28i32);
    pub const IsoB4Envelope: Self = Self(29i32);
    pub const IsoB5Envelope: Self = Self(30i32);
    pub const IsoB5Extra: Self = Self(31i32);
    pub const IsoB7: Self = Self(32i32);
    pub const IsoB8: Self = Self(33i32);
    pub const IsoB9: Self = Self(34i32);
    pub const IsoC0: Self = Self(35i32);
    pub const IsoC1: Self = Self(36i32);
    pub const IsoC10: Self = Self(37i32);
    pub const IsoC2: Self = Self(38i32);
    pub const IsoC3: Self = Self(39i32);
    pub const IsoC3Envelope: Self = Self(40i32);
    pub const IsoC4: Self = Self(41i32);
    pub const IsoC4Envelope: Self = Self(42i32);
    pub const IsoC5: Self = Self(43i32);
    pub const IsoC5Envelope: Self = Self(44i32);
    pub const IsoC6: Self = Self(45i32);
    pub const IsoC6C5Envelope: Self = Self(46i32);
    pub const IsoC6Envelope: Self = Self(47i32);
    pub const IsoC7: Self = Self(48i32);
    pub const IsoC8: Self = Self(49i32);
    pub const IsoC9: Self = Self(50i32);
    pub const IsoDLEnvelope: Self = Self(51i32);
    pub const IsoDLEnvelopeRotated: Self = Self(52i32);
    pub const IsoSRA3: Self = Self(53i32);
    pub const Japan2LPhoto: Self = Self(54i32);
    pub const JapanChou3Envelope: Self = Self(55i32);
    pub const JapanChou3EnvelopeRotated: Self = Self(56i32);
    pub const JapanChou4Envelope: Self = Self(57i32);
    pub const JapanChou4EnvelopeRotated: Self = Self(58i32);
    pub const JapanDoubleHagakiPostcard: Self = Self(59i32);
    pub const JapanDoubleHagakiPostcardRotated: Self = Self(60i32);
    pub const JapanHagakiPostcard: Self = Self(61i32);
    pub const JapanHagakiPostcardRotated: Self = Self(62i32);
    pub const JapanKaku2Envelope: Self = Self(63i32);
    pub const JapanKaku2EnvelopeRotated: Self = Self(64i32);
    pub const JapanKaku3Envelope: Self = Self(65i32);
    pub const JapanKaku3EnvelopeRotated: Self = Self(66i32);
    pub const JapanLPhoto: Self = Self(67i32);
    pub const JapanQuadrupleHagakiPostcard: Self = Self(68i32);
    pub const JapanYou1Envelope: Self = Self(69i32);
    pub const JapanYou2Envelope: Self = Self(70i32);
    pub const JapanYou3Envelope: Self = Self(71i32);
    pub const JapanYou4Envelope: Self = Self(72i32);
    pub const JapanYou4EnvelopeRotated: Self = Self(73i32);
    pub const JapanYou6Envelope: Self = Self(74i32);
    pub const JapanYou6EnvelopeRotated: Self = Self(75i32);
    pub const JisB0: Self = Self(76i32);
    pub const JisB1: Self = Self(77i32);
    pub const JisB10: Self = Self(78i32);
    pub const JisB2: Self = Self(79i32);
    pub const JisB3: Self = Self(80i32);
    pub const JisB4: Self = Self(81i32);
    pub const JisB4Rotated: Self = Self(82i32);
    pub const JisB5: Self = Self(83i32);
    pub const JisB5Rotated: Self = Self(84i32);
    pub const JisB6: Self = Self(85i32);
    pub const JisB6Rotated: Self = Self(86i32);
    pub const JisB7: Self = Self(87i32);
    pub const JisB8: Self = Self(88i32);
    pub const JisB9: Self = Self(89i32);
    pub const NorthAmerica10x11: Self = Self(90i32);
    pub const NorthAmerica10x12: Self = Self(91i32);
    pub const NorthAmerica10x14: Self = Self(92i32);
    pub const NorthAmerica11x17: Self = Self(93i32);
    pub const NorthAmerica14x17: Self = Self(94i32);
    pub const NorthAmerica4x6: Self = Self(95i32);
    pub const NorthAmerica4x8: Self = Self(96i32);
    pub const NorthAmerica5x7: Self = Self(97i32);
    pub const NorthAmerica8x10: Self = Self(98i32);
    pub const NorthAmerica9x11: Self = Self(99i32);
    pub const NorthAmericaArchitectureASheet: Self = Self(100i32);
    pub const NorthAmericaArchitectureBSheet: Self = Self(101i32);
    pub const NorthAmericaArchitectureCSheet: Self = Self(102i32);
    pub const NorthAmericaArchitectureDSheet: Self = Self(103i32);
    pub const NorthAmericaArchitectureESheet: Self = Self(104i32);
    pub const NorthAmericaCSheet: Self = Self(105i32);
    pub const NorthAmericaDSheet: Self = Self(106i32);
    pub const NorthAmericaESheet: Self = Self(107i32);
    pub const NorthAmericaExecutive: Self = Self(108i32);
    pub const NorthAmericaGermanLegalFanfold: Self = Self(109i32);
    pub const NorthAmericaGermanStandardFanfold: Self = Self(110i32);
    pub const NorthAmericaLegal: Self = Self(111i32);
    pub const NorthAmericaLegalExtra: Self = Self(112i32);
    pub const NorthAmericaLetter: Self = Self(113i32);
    pub const NorthAmericaLetterExtra: Self = Self(114i32);
    pub const NorthAmericaLetterPlus: Self = Self(115i32);
    pub const NorthAmericaLetterRotated: Self = Self(116i32);
    pub const NorthAmericaMonarchEnvelope: Self = Self(117i32);
    pub const NorthAmericaNote: Self = Self(118i32);
    pub const NorthAmericaNumber10Envelope: Self = Self(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: Self = Self(120i32);
    pub const NorthAmericaNumber11Envelope: Self = Self(121i32);
    pub const NorthAmericaNumber12Envelope: Self = Self(122i32);
    pub const NorthAmericaNumber14Envelope: Self = Self(123i32);
    pub const NorthAmericaNumber9Envelope: Self = Self(124i32);
    pub const NorthAmericaPersonalEnvelope: Self = Self(125i32);
    pub const NorthAmericaQuarto: Self = Self(126i32);
    pub const NorthAmericaStatement: Self = Self(127i32);
    pub const NorthAmericaSuperA: Self = Self(128i32);
    pub const NorthAmericaSuperB: Self = Self(129i32);
    pub const NorthAmericaTabloid: Self = Self(130i32);
    pub const NorthAmericaTabloidExtra: Self = Self(131i32);
    pub const OtherMetricA3Plus: Self = Self(132i32);
    pub const OtherMetricA4Plus: Self = Self(133i32);
    pub const OtherMetricFolio: Self = Self(134i32);
    pub const OtherMetricInviteEnvelope: Self = Self(135i32);
    pub const OtherMetricItalianEnvelope: Self = Self(136i32);
    pub const Prc10Envelope: Self = Self(137i32);
    pub const Prc10EnvelopeRotated: Self = Self(138i32);
    pub const Prc16K: Self = Self(139i32);
    pub const Prc16KRotated: Self = Self(140i32);
    pub const Prc1Envelope: Self = Self(141i32);
    pub const Prc1EnvelopeRotated: Self = Self(142i32);
    pub const Prc2Envelope: Self = Self(143i32);
    pub const Prc2EnvelopeRotated: Self = Self(144i32);
    pub const Prc32K: Self = Self(145i32);
    pub const Prc32KBig: Self = Self(146i32);
    pub const Prc32KRotated: Self = Self(147i32);
    pub const Prc3Envelope: Self = Self(148i32);
    pub const Prc3EnvelopeRotated: Self = Self(149i32);
    pub const Prc4Envelope: Self = Self(150i32);
    pub const Prc4EnvelopeRotated: Self = Self(151i32);
    pub const Prc5Envelope: Self = Self(152i32);
    pub const Prc5EnvelopeRotated: Self = Self(153i32);
    pub const Prc6Envelope: Self = Self(154i32);
    pub const Prc6EnvelopeRotated: Self = Self(155i32);
    pub const Prc7Envelope: Self = Self(156i32);
    pub const Prc7EnvelopeRotated: Self = Self(157i32);
    pub const Prc8Envelope: Self = Self(158i32);
    pub const Prc8EnvelopeRotated: Self = Self(159i32);
    pub const Prc9Envelope: Self = Self(160i32);
    pub const Prc9EnvelopeRotated: Self = Self(161i32);
    pub const Roll04Inch: Self = Self(162i32);
    pub const Roll06Inch: Self = Self(163i32);
    pub const Roll08Inch: Self = Self(164i32);
    pub const Roll12Inch: Self = Self(165i32);
    pub const Roll15Inch: Self = Self(166i32);
    pub const Roll18Inch: Self = Self(167i32);
    pub const Roll22Inch: Self = Self(168i32);
    pub const Roll24Inch: Self = Self(169i32);
    pub const Roll30Inch: Self = Self(170i32);
    pub const Roll36Inch: Self = Self(171i32);
    pub const Roll54Inch: Self = Self(172i32);
}
impl ::core::marker::Copy for PrintMediaSize {}
impl ::core::clone::Clone for PrintMediaSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const AutoSelect: Self = Self(3i32);
    pub const Archival: Self = Self(4i32);
    pub const BackPrintFilm: Self = Self(5i32);
    pub const Bond: Self = Self(6i32);
    pub const CardStock: Self = Self(7i32);
    pub const Continuous: Self = Self(8i32);
    pub const EnvelopePlain: Self = Self(9i32);
    pub const EnvelopeWindow: Self = Self(10i32);
    pub const Fabric: Self = Self(11i32);
    pub const HighResolution: Self = Self(12i32);
    pub const Label: Self = Self(13i32);
    pub const MultiLayerForm: Self = Self(14i32);
    pub const MultiPartForm: Self = Self(15i32);
    pub const Photographic: Self = Self(16i32);
    pub const PhotographicFilm: Self = Self(17i32);
    pub const PhotographicGlossy: Self = Self(18i32);
    pub const PhotographicHighGloss: Self = Self(19i32);
    pub const PhotographicMatte: Self = Self(20i32);
    pub const PhotographicSatin: Self = Self(21i32);
    pub const PhotographicSemiGloss: Self = Self(22i32);
    pub const Plain: Self = Self(23i32);
    pub const Screen: Self = Self(24i32);
    pub const ScreenPaged: Self = Self(25i32);
    pub const Stationery: Self = Self(26i32);
    pub const TabStockFull: Self = Self(27i32);
    pub const TabStockPreCut: Self = Self(28i32);
    pub const Transparency: Self = Self(29i32);
    pub const TShirtTransfer: Self = Self(30i32);
    pub const None: Self = Self(31i32);
}
impl ::core::marker::Copy for PrintMediaType {}
impl ::core::clone::Clone for PrintMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const PortraitFlipped: Self = Self(4i32);
    pub const Landscape: Self = Self(5i32);
    pub const LandscapeFlipped: Self = Self(6i32);
}
impl ::core::marker::Copy for PrintOrientation {}
impl ::core::clone::Clone for PrintOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription {
    pub PageSize: super::super::Foundation::Size,
    pub ImageableRect: super::super::Foundation::Rect,
    pub DpiX: u32,
    pub DpiY: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for PrintPageDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintPageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintPageInfo {}
impl ::core::clone::Clone for PrintPageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintPageRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintPageRange {}
impl ::core::clone::Clone for PrintPageRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintPageRangeOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintPageRangeOptions {}
impl ::core::clone::Clone for PrintPageRangeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Automatic: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
    pub const Fax: Self = Self(5i32);
    pub const High: Self = Self(6i32);
    pub const Normal: Self = Self(7i32);
    pub const Photographic: Self = Self(8i32);
    pub const Text: Self = Self(9i32);
}
impl ::core::marker::Copy for PrintQuality {}
impl ::core::clone::Clone for PrintQuality {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const StapleTopLeft: Self = Self(4i32);
    pub const StapleTopRight: Self = Self(5i32);
    pub const StapleBottomLeft: Self = Self(6i32);
    pub const StapleBottomRight: Self = Self(7i32);
    pub const StapleDualLeft: Self = Self(8i32);
    pub const StapleDualRight: Self = Self(9i32);
    pub const StapleDualTop: Self = Self(10i32);
    pub const StapleDualBottom: Self = Self(11i32);
    pub const SaddleStitch: Self = Self(12i32);
}
impl ::core::marker::Copy for PrintStaple {}
impl ::core::clone::Clone for PrintStaple {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTask {}
impl ::core::clone::Clone for PrintTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskCompletedEventArgs {}
impl ::core::clone::Clone for PrintTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Submitted: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintTaskCompletion {}
impl ::core::clone::Clone for PrintTaskCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskOptions {}
impl ::core::clone::Clone for PrintTaskOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskProgressingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskProgressingEventArgs {}
impl ::core::clone::Clone for PrintTaskProgressingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskRequest {}
impl ::core::clone::Clone for PrintTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskRequestedDeferral {}
impl ::core::clone::Clone for PrintTaskRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskRequestedEventArgs {}
impl ::core::clone::Clone for PrintTaskRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskSourceRequestedArgs {}
impl ::core::clone::Clone for PrintTaskSourceRequestedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskSourceRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskSourceRequestedDeferral {}
impl ::core::clone::Clone for PrintTaskSourceRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskSourceRequestedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskSourceRequestedHandler {}
impl ::core::clone::Clone for PrintTaskSourceRequestedHandler {
    fn clone(&self) -> Self {
        *self
    }
}

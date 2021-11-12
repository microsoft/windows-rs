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
#[repr(transparent)]
pub struct IPrintManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintManagerStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintManagerStatic2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintPageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintPageRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintPageRangeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintPageRangeOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTask2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionsCore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionsCoreUIConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskProgressingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskSourceRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskTargetDeviceSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardPrintTaskOptionsStatic3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: PrintBinding = PrintBinding(0i32);
    pub const NotAvailable: PrintBinding = PrintBinding(1i32);
    pub const PrinterCustom: PrintBinding = PrintBinding(2i32);
    pub const None: PrintBinding = PrintBinding(3i32);
    pub const Bale: PrintBinding = PrintBinding(4i32);
    pub const BindBottom: PrintBinding = PrintBinding(5i32);
    pub const BindLeft: PrintBinding = PrintBinding(6i32);
    pub const BindRight: PrintBinding = PrintBinding(7i32);
    pub const BindTop: PrintBinding = PrintBinding(8i32);
    pub const Booklet: PrintBinding = PrintBinding(9i32);
    pub const EdgeStitchBottom: PrintBinding = PrintBinding(10i32);
    pub const EdgeStitchLeft: PrintBinding = PrintBinding(11i32);
    pub const EdgeStitchRight: PrintBinding = PrintBinding(12i32);
    pub const EdgeStitchTop: PrintBinding = PrintBinding(13i32);
    pub const Fold: PrintBinding = PrintBinding(14i32);
    pub const JogOffset: PrintBinding = PrintBinding(15i32);
    pub const Trim: PrintBinding = PrintBinding(16i32);
}
#[repr(transparent)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: PrintBordering = PrintBordering(0i32);
    pub const NotAvailable: PrintBordering = PrintBordering(1i32);
    pub const PrinterCustom: PrintBordering = PrintBordering(2i32);
    pub const Bordered: PrintBordering = PrintBordering(3i32);
    pub const Borderless: PrintBordering = PrintBordering(4i32);
}
#[repr(transparent)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: PrintCollation = PrintCollation(0i32);
    pub const NotAvailable: PrintCollation = PrintCollation(1i32);
    pub const PrinterCustom: PrintCollation = PrintCollation(2i32);
    pub const Collated: PrintCollation = PrintCollation(3i32);
    pub const Uncollated: PrintCollation = PrintCollation(4i32);
}
#[repr(transparent)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: PrintColorMode = PrintColorMode(0i32);
    pub const NotAvailable: PrintColorMode = PrintColorMode(1i32);
    pub const PrinterCustom: PrintColorMode = PrintColorMode(2i32);
    pub const Color: PrintColorMode = PrintColorMode(3i32);
    pub const Grayscale: PrintColorMode = PrintColorMode(4i32);
    pub const Monochrome: PrintColorMode = PrintColorMode(5i32);
}
#[repr(transparent)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: PrintDuplex = PrintDuplex(0i32);
    pub const NotAvailable: PrintDuplex = PrintDuplex(1i32);
    pub const PrinterCustom: PrintDuplex = PrintDuplex(2i32);
    pub const OneSided: PrintDuplex = PrintDuplex(3i32);
    pub const TwoSidedShortEdge: PrintDuplex = PrintDuplex(4i32);
    pub const TwoSidedLongEdge: PrintDuplex = PrintDuplex(5i32);
}
#[repr(transparent)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: PrintHolePunch = PrintHolePunch(0i32);
    pub const NotAvailable: PrintHolePunch = PrintHolePunch(1i32);
    pub const PrinterCustom: PrintHolePunch = PrintHolePunch(2i32);
    pub const None: PrintHolePunch = PrintHolePunch(3i32);
    pub const LeftEdge: PrintHolePunch = PrintHolePunch(4i32);
    pub const RightEdge: PrintHolePunch = PrintHolePunch(5i32);
    pub const TopEdge: PrintHolePunch = PrintHolePunch(6i32);
    pub const BottomEdge: PrintHolePunch = PrintHolePunch(7i32);
}
#[repr(transparent)]
pub struct PrintManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: PrintMediaSize = PrintMediaSize(0i32);
    pub const NotAvailable: PrintMediaSize = PrintMediaSize(1i32);
    pub const PrinterCustom: PrintMediaSize = PrintMediaSize(2i32);
    pub const BusinessCard: PrintMediaSize = PrintMediaSize(3i32);
    pub const CreditCard: PrintMediaSize = PrintMediaSize(4i32);
    pub const IsoA0: PrintMediaSize = PrintMediaSize(5i32);
    pub const IsoA1: PrintMediaSize = PrintMediaSize(6i32);
    pub const IsoA10: PrintMediaSize = PrintMediaSize(7i32);
    pub const IsoA2: PrintMediaSize = PrintMediaSize(8i32);
    pub const IsoA3: PrintMediaSize = PrintMediaSize(9i32);
    pub const IsoA3Extra: PrintMediaSize = PrintMediaSize(10i32);
    pub const IsoA3Rotated: PrintMediaSize = PrintMediaSize(11i32);
    pub const IsoA4: PrintMediaSize = PrintMediaSize(12i32);
    pub const IsoA4Extra: PrintMediaSize = PrintMediaSize(13i32);
    pub const IsoA4Rotated: PrintMediaSize = PrintMediaSize(14i32);
    pub const IsoA5: PrintMediaSize = PrintMediaSize(15i32);
    pub const IsoA5Extra: PrintMediaSize = PrintMediaSize(16i32);
    pub const IsoA5Rotated: PrintMediaSize = PrintMediaSize(17i32);
    pub const IsoA6: PrintMediaSize = PrintMediaSize(18i32);
    pub const IsoA6Rotated: PrintMediaSize = PrintMediaSize(19i32);
    pub const IsoA7: PrintMediaSize = PrintMediaSize(20i32);
    pub const IsoA8: PrintMediaSize = PrintMediaSize(21i32);
    pub const IsoA9: PrintMediaSize = PrintMediaSize(22i32);
    pub const IsoB0: PrintMediaSize = PrintMediaSize(23i32);
    pub const IsoB1: PrintMediaSize = PrintMediaSize(24i32);
    pub const IsoB10: PrintMediaSize = PrintMediaSize(25i32);
    pub const IsoB2: PrintMediaSize = PrintMediaSize(26i32);
    pub const IsoB3: PrintMediaSize = PrintMediaSize(27i32);
    pub const IsoB4: PrintMediaSize = PrintMediaSize(28i32);
    pub const IsoB4Envelope: PrintMediaSize = PrintMediaSize(29i32);
    pub const IsoB5Envelope: PrintMediaSize = PrintMediaSize(30i32);
    pub const IsoB5Extra: PrintMediaSize = PrintMediaSize(31i32);
    pub const IsoB7: PrintMediaSize = PrintMediaSize(32i32);
    pub const IsoB8: PrintMediaSize = PrintMediaSize(33i32);
    pub const IsoB9: PrintMediaSize = PrintMediaSize(34i32);
    pub const IsoC0: PrintMediaSize = PrintMediaSize(35i32);
    pub const IsoC1: PrintMediaSize = PrintMediaSize(36i32);
    pub const IsoC10: PrintMediaSize = PrintMediaSize(37i32);
    pub const IsoC2: PrintMediaSize = PrintMediaSize(38i32);
    pub const IsoC3: PrintMediaSize = PrintMediaSize(39i32);
    pub const IsoC3Envelope: PrintMediaSize = PrintMediaSize(40i32);
    pub const IsoC4: PrintMediaSize = PrintMediaSize(41i32);
    pub const IsoC4Envelope: PrintMediaSize = PrintMediaSize(42i32);
    pub const IsoC5: PrintMediaSize = PrintMediaSize(43i32);
    pub const IsoC5Envelope: PrintMediaSize = PrintMediaSize(44i32);
    pub const IsoC6: PrintMediaSize = PrintMediaSize(45i32);
    pub const IsoC6C5Envelope: PrintMediaSize = PrintMediaSize(46i32);
    pub const IsoC6Envelope: PrintMediaSize = PrintMediaSize(47i32);
    pub const IsoC7: PrintMediaSize = PrintMediaSize(48i32);
    pub const IsoC8: PrintMediaSize = PrintMediaSize(49i32);
    pub const IsoC9: PrintMediaSize = PrintMediaSize(50i32);
    pub const IsoDLEnvelope: PrintMediaSize = PrintMediaSize(51i32);
    pub const IsoDLEnvelopeRotated: PrintMediaSize = PrintMediaSize(52i32);
    pub const IsoSRA3: PrintMediaSize = PrintMediaSize(53i32);
    pub const Japan2LPhoto: PrintMediaSize = PrintMediaSize(54i32);
    pub const JapanChou3Envelope: PrintMediaSize = PrintMediaSize(55i32);
    pub const JapanChou3EnvelopeRotated: PrintMediaSize = PrintMediaSize(56i32);
    pub const JapanChou4Envelope: PrintMediaSize = PrintMediaSize(57i32);
    pub const JapanChou4EnvelopeRotated: PrintMediaSize = PrintMediaSize(58i32);
    pub const JapanDoubleHagakiPostcard: PrintMediaSize = PrintMediaSize(59i32);
    pub const JapanDoubleHagakiPostcardRotated: PrintMediaSize = PrintMediaSize(60i32);
    pub const JapanHagakiPostcard: PrintMediaSize = PrintMediaSize(61i32);
    pub const JapanHagakiPostcardRotated: PrintMediaSize = PrintMediaSize(62i32);
    pub const JapanKaku2Envelope: PrintMediaSize = PrintMediaSize(63i32);
    pub const JapanKaku2EnvelopeRotated: PrintMediaSize = PrintMediaSize(64i32);
    pub const JapanKaku3Envelope: PrintMediaSize = PrintMediaSize(65i32);
    pub const JapanKaku3EnvelopeRotated: PrintMediaSize = PrintMediaSize(66i32);
    pub const JapanLPhoto: PrintMediaSize = PrintMediaSize(67i32);
    pub const JapanQuadrupleHagakiPostcard: PrintMediaSize = PrintMediaSize(68i32);
    pub const JapanYou1Envelope: PrintMediaSize = PrintMediaSize(69i32);
    pub const JapanYou2Envelope: PrintMediaSize = PrintMediaSize(70i32);
    pub const JapanYou3Envelope: PrintMediaSize = PrintMediaSize(71i32);
    pub const JapanYou4Envelope: PrintMediaSize = PrintMediaSize(72i32);
    pub const JapanYou4EnvelopeRotated: PrintMediaSize = PrintMediaSize(73i32);
    pub const JapanYou6Envelope: PrintMediaSize = PrintMediaSize(74i32);
    pub const JapanYou6EnvelopeRotated: PrintMediaSize = PrintMediaSize(75i32);
    pub const JisB0: PrintMediaSize = PrintMediaSize(76i32);
    pub const JisB1: PrintMediaSize = PrintMediaSize(77i32);
    pub const JisB10: PrintMediaSize = PrintMediaSize(78i32);
    pub const JisB2: PrintMediaSize = PrintMediaSize(79i32);
    pub const JisB3: PrintMediaSize = PrintMediaSize(80i32);
    pub const JisB4: PrintMediaSize = PrintMediaSize(81i32);
    pub const JisB4Rotated: PrintMediaSize = PrintMediaSize(82i32);
    pub const JisB5: PrintMediaSize = PrintMediaSize(83i32);
    pub const JisB5Rotated: PrintMediaSize = PrintMediaSize(84i32);
    pub const JisB6: PrintMediaSize = PrintMediaSize(85i32);
    pub const JisB6Rotated: PrintMediaSize = PrintMediaSize(86i32);
    pub const JisB7: PrintMediaSize = PrintMediaSize(87i32);
    pub const JisB8: PrintMediaSize = PrintMediaSize(88i32);
    pub const JisB9: PrintMediaSize = PrintMediaSize(89i32);
    pub const NorthAmerica10x11: PrintMediaSize = PrintMediaSize(90i32);
    pub const NorthAmerica10x12: PrintMediaSize = PrintMediaSize(91i32);
    pub const NorthAmerica10x14: PrintMediaSize = PrintMediaSize(92i32);
    pub const NorthAmerica11x17: PrintMediaSize = PrintMediaSize(93i32);
    pub const NorthAmerica14x17: PrintMediaSize = PrintMediaSize(94i32);
    pub const NorthAmerica4x6: PrintMediaSize = PrintMediaSize(95i32);
    pub const NorthAmerica4x8: PrintMediaSize = PrintMediaSize(96i32);
    pub const NorthAmerica5x7: PrintMediaSize = PrintMediaSize(97i32);
    pub const NorthAmerica8x10: PrintMediaSize = PrintMediaSize(98i32);
    pub const NorthAmerica9x11: PrintMediaSize = PrintMediaSize(99i32);
    pub const NorthAmericaArchitectureASheet: PrintMediaSize = PrintMediaSize(100i32);
    pub const NorthAmericaArchitectureBSheet: PrintMediaSize = PrintMediaSize(101i32);
    pub const NorthAmericaArchitectureCSheet: PrintMediaSize = PrintMediaSize(102i32);
    pub const NorthAmericaArchitectureDSheet: PrintMediaSize = PrintMediaSize(103i32);
    pub const NorthAmericaArchitectureESheet: PrintMediaSize = PrintMediaSize(104i32);
    pub const NorthAmericaCSheet: PrintMediaSize = PrintMediaSize(105i32);
    pub const NorthAmericaDSheet: PrintMediaSize = PrintMediaSize(106i32);
    pub const NorthAmericaESheet: PrintMediaSize = PrintMediaSize(107i32);
    pub const NorthAmericaExecutive: PrintMediaSize = PrintMediaSize(108i32);
    pub const NorthAmericaGermanLegalFanfold: PrintMediaSize = PrintMediaSize(109i32);
    pub const NorthAmericaGermanStandardFanfold: PrintMediaSize = PrintMediaSize(110i32);
    pub const NorthAmericaLegal: PrintMediaSize = PrintMediaSize(111i32);
    pub const NorthAmericaLegalExtra: PrintMediaSize = PrintMediaSize(112i32);
    pub const NorthAmericaLetter: PrintMediaSize = PrintMediaSize(113i32);
    pub const NorthAmericaLetterExtra: PrintMediaSize = PrintMediaSize(114i32);
    pub const NorthAmericaLetterPlus: PrintMediaSize = PrintMediaSize(115i32);
    pub const NorthAmericaLetterRotated: PrintMediaSize = PrintMediaSize(116i32);
    pub const NorthAmericaMonarchEnvelope: PrintMediaSize = PrintMediaSize(117i32);
    pub const NorthAmericaNote: PrintMediaSize = PrintMediaSize(118i32);
    pub const NorthAmericaNumber10Envelope: PrintMediaSize = PrintMediaSize(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: PrintMediaSize = PrintMediaSize(120i32);
    pub const NorthAmericaNumber11Envelope: PrintMediaSize = PrintMediaSize(121i32);
    pub const NorthAmericaNumber12Envelope: PrintMediaSize = PrintMediaSize(122i32);
    pub const NorthAmericaNumber14Envelope: PrintMediaSize = PrintMediaSize(123i32);
    pub const NorthAmericaNumber9Envelope: PrintMediaSize = PrintMediaSize(124i32);
    pub const NorthAmericaPersonalEnvelope: PrintMediaSize = PrintMediaSize(125i32);
    pub const NorthAmericaQuarto: PrintMediaSize = PrintMediaSize(126i32);
    pub const NorthAmericaStatement: PrintMediaSize = PrintMediaSize(127i32);
    pub const NorthAmericaSuperA: PrintMediaSize = PrintMediaSize(128i32);
    pub const NorthAmericaSuperB: PrintMediaSize = PrintMediaSize(129i32);
    pub const NorthAmericaTabloid: PrintMediaSize = PrintMediaSize(130i32);
    pub const NorthAmericaTabloidExtra: PrintMediaSize = PrintMediaSize(131i32);
    pub const OtherMetricA3Plus: PrintMediaSize = PrintMediaSize(132i32);
    pub const OtherMetricA4Plus: PrintMediaSize = PrintMediaSize(133i32);
    pub const OtherMetricFolio: PrintMediaSize = PrintMediaSize(134i32);
    pub const OtherMetricInviteEnvelope: PrintMediaSize = PrintMediaSize(135i32);
    pub const OtherMetricItalianEnvelope: PrintMediaSize = PrintMediaSize(136i32);
    pub const Prc10Envelope: PrintMediaSize = PrintMediaSize(137i32);
    pub const Prc10EnvelopeRotated: PrintMediaSize = PrintMediaSize(138i32);
    pub const Prc16K: PrintMediaSize = PrintMediaSize(139i32);
    pub const Prc16KRotated: PrintMediaSize = PrintMediaSize(140i32);
    pub const Prc1Envelope: PrintMediaSize = PrintMediaSize(141i32);
    pub const Prc1EnvelopeRotated: PrintMediaSize = PrintMediaSize(142i32);
    pub const Prc2Envelope: PrintMediaSize = PrintMediaSize(143i32);
    pub const Prc2EnvelopeRotated: PrintMediaSize = PrintMediaSize(144i32);
    pub const Prc32K: PrintMediaSize = PrintMediaSize(145i32);
    pub const Prc32KBig: PrintMediaSize = PrintMediaSize(146i32);
    pub const Prc32KRotated: PrintMediaSize = PrintMediaSize(147i32);
    pub const Prc3Envelope: PrintMediaSize = PrintMediaSize(148i32);
    pub const Prc3EnvelopeRotated: PrintMediaSize = PrintMediaSize(149i32);
    pub const Prc4Envelope: PrintMediaSize = PrintMediaSize(150i32);
    pub const Prc4EnvelopeRotated: PrintMediaSize = PrintMediaSize(151i32);
    pub const Prc5Envelope: PrintMediaSize = PrintMediaSize(152i32);
    pub const Prc5EnvelopeRotated: PrintMediaSize = PrintMediaSize(153i32);
    pub const Prc6Envelope: PrintMediaSize = PrintMediaSize(154i32);
    pub const Prc6EnvelopeRotated: PrintMediaSize = PrintMediaSize(155i32);
    pub const Prc7Envelope: PrintMediaSize = PrintMediaSize(156i32);
    pub const Prc7EnvelopeRotated: PrintMediaSize = PrintMediaSize(157i32);
    pub const Prc8Envelope: PrintMediaSize = PrintMediaSize(158i32);
    pub const Prc8EnvelopeRotated: PrintMediaSize = PrintMediaSize(159i32);
    pub const Prc9Envelope: PrintMediaSize = PrintMediaSize(160i32);
    pub const Prc9EnvelopeRotated: PrintMediaSize = PrintMediaSize(161i32);
    pub const Roll04Inch: PrintMediaSize = PrintMediaSize(162i32);
    pub const Roll06Inch: PrintMediaSize = PrintMediaSize(163i32);
    pub const Roll08Inch: PrintMediaSize = PrintMediaSize(164i32);
    pub const Roll12Inch: PrintMediaSize = PrintMediaSize(165i32);
    pub const Roll15Inch: PrintMediaSize = PrintMediaSize(166i32);
    pub const Roll18Inch: PrintMediaSize = PrintMediaSize(167i32);
    pub const Roll22Inch: PrintMediaSize = PrintMediaSize(168i32);
    pub const Roll24Inch: PrintMediaSize = PrintMediaSize(169i32);
    pub const Roll30Inch: PrintMediaSize = PrintMediaSize(170i32);
    pub const Roll36Inch: PrintMediaSize = PrintMediaSize(171i32);
    pub const Roll54Inch: PrintMediaSize = PrintMediaSize(172i32);
}
#[repr(transparent)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: PrintMediaType = PrintMediaType(0i32);
    pub const NotAvailable: PrintMediaType = PrintMediaType(1i32);
    pub const PrinterCustom: PrintMediaType = PrintMediaType(2i32);
    pub const AutoSelect: PrintMediaType = PrintMediaType(3i32);
    pub const Archival: PrintMediaType = PrintMediaType(4i32);
    pub const BackPrintFilm: PrintMediaType = PrintMediaType(5i32);
    pub const Bond: PrintMediaType = PrintMediaType(6i32);
    pub const CardStock: PrintMediaType = PrintMediaType(7i32);
    pub const Continuous: PrintMediaType = PrintMediaType(8i32);
    pub const EnvelopePlain: PrintMediaType = PrintMediaType(9i32);
    pub const EnvelopeWindow: PrintMediaType = PrintMediaType(10i32);
    pub const Fabric: PrintMediaType = PrintMediaType(11i32);
    pub const HighResolution: PrintMediaType = PrintMediaType(12i32);
    pub const Label: PrintMediaType = PrintMediaType(13i32);
    pub const MultiLayerForm: PrintMediaType = PrintMediaType(14i32);
    pub const MultiPartForm: PrintMediaType = PrintMediaType(15i32);
    pub const Photographic: PrintMediaType = PrintMediaType(16i32);
    pub const PhotographicFilm: PrintMediaType = PrintMediaType(17i32);
    pub const PhotographicGlossy: PrintMediaType = PrintMediaType(18i32);
    pub const PhotographicHighGloss: PrintMediaType = PrintMediaType(19i32);
    pub const PhotographicMatte: PrintMediaType = PrintMediaType(20i32);
    pub const PhotographicSatin: PrintMediaType = PrintMediaType(21i32);
    pub const PhotographicSemiGloss: PrintMediaType = PrintMediaType(22i32);
    pub const Plain: PrintMediaType = PrintMediaType(23i32);
    pub const Screen: PrintMediaType = PrintMediaType(24i32);
    pub const ScreenPaged: PrintMediaType = PrintMediaType(25i32);
    pub const Stationery: PrintMediaType = PrintMediaType(26i32);
    pub const TabStockFull: PrintMediaType = PrintMediaType(27i32);
    pub const TabStockPreCut: PrintMediaType = PrintMediaType(28i32);
    pub const Transparency: PrintMediaType = PrintMediaType(29i32);
    pub const TShirtTransfer: PrintMediaType = PrintMediaType(30i32);
    pub const None: PrintMediaType = PrintMediaType(31i32);
}
#[repr(transparent)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: PrintOrientation = PrintOrientation(0i32);
    pub const NotAvailable: PrintOrientation = PrintOrientation(1i32);
    pub const PrinterCustom: PrintOrientation = PrintOrientation(2i32);
    pub const Portrait: PrintOrientation = PrintOrientation(3i32);
    pub const PortraitFlipped: PrintOrientation = PrintOrientation(4i32);
    pub const Landscape: PrintOrientation = PrintOrientation(5i32);
    pub const LandscapeFlipped: PrintOrientation = PrintOrientation(6i32);
}
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct PrintPageDescription(i32);
#[repr(transparent)]
pub struct PrintPageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintPageRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintPageRangeOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: PrintQuality = PrintQuality(0i32);
    pub const NotAvailable: PrintQuality = PrintQuality(1i32);
    pub const PrinterCustom: PrintQuality = PrintQuality(2i32);
    pub const Automatic: PrintQuality = PrintQuality(3i32);
    pub const Draft: PrintQuality = PrintQuality(4i32);
    pub const Fax: PrintQuality = PrintQuality(5i32);
    pub const High: PrintQuality = PrintQuality(6i32);
    pub const Normal: PrintQuality = PrintQuality(7i32);
    pub const Photographic: PrintQuality = PrintQuality(8i32);
    pub const Text: PrintQuality = PrintQuality(9i32);
}
#[repr(transparent)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: PrintStaple = PrintStaple(0i32);
    pub const NotAvailable: PrintStaple = PrintStaple(1i32);
    pub const PrinterCustom: PrintStaple = PrintStaple(2i32);
    pub const None: PrintStaple = PrintStaple(3i32);
    pub const StapleTopLeft: PrintStaple = PrintStaple(4i32);
    pub const StapleTopRight: PrintStaple = PrintStaple(5i32);
    pub const StapleBottomLeft: PrintStaple = PrintStaple(6i32);
    pub const StapleBottomRight: PrintStaple = PrintStaple(7i32);
    pub const StapleDualLeft: PrintStaple = PrintStaple(8i32);
    pub const StapleDualRight: PrintStaple = PrintStaple(9i32);
    pub const StapleDualTop: PrintStaple = PrintStaple(10i32);
    pub const StapleDualBottom: PrintStaple = PrintStaple(11i32);
    pub const SaddleStitch: PrintStaple = PrintStaple(12i32);
}
#[repr(transparent)]
pub struct PrintTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: PrintTaskCompletion = PrintTaskCompletion(0i32);
    pub const Canceled: PrintTaskCompletion = PrintTaskCompletion(1i32);
    pub const Failed: PrintTaskCompletion = PrintTaskCompletion(2i32);
    pub const Submitted: PrintTaskCompletion = PrintTaskCompletion(3i32);
}
#[repr(transparent)]
pub struct PrintTaskOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskProgressingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskSourceRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskSourceRequestedHandler(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types)]
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
pub struct IPrintDocumentSource(pub *mut ::core::ffi::c_void);
pub struct IPrintManager(pub *mut ::core::ffi::c_void);
pub struct IPrintManagerStatic(pub *mut ::core::ffi::c_void);
pub struct IPrintManagerStatic2(pub *mut ::core::ffi::c_void);
pub struct IPrintPageInfo(pub *mut ::core::ffi::c_void);
pub struct IPrintPageRange(pub *mut ::core::ffi::c_void);
pub struct IPrintPageRangeFactory(pub *mut ::core::ffi::c_void);
pub struct IPrintPageRangeOptions(pub *mut ::core::ffi::c_void);
pub struct IPrintTask(pub *mut ::core::ffi::c_void);
pub struct IPrintTask2(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskOptions(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskOptions2(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskOptionsCore(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskOptionsCoreProperties(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskOptionsCoreUIConfiguration(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskProgressingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskRequest(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskRequestedDeferral(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskSourceRequestedArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskSourceRequestedDeferral(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskTargetDeviceSupport(pub *mut ::core::ffi::c_void);
pub struct IStandardPrintTaskOptionsStatic(pub *mut ::core::ffi::c_void);
pub struct IStandardPrintTaskOptionsStatic2(pub *mut ::core::ffi::c_void);
pub struct IStandardPrintTaskOptionsStatic3(pub *mut ::core::ffi::c_void);
pub struct PrintBinding(i32);
pub struct PrintBordering(i32);
pub struct PrintCollation(i32);
pub struct PrintColorMode(i32);
pub struct PrintDuplex(i32);
pub struct PrintHolePunch(i32);
pub struct PrintManager(i32);
pub struct PrintMediaSize(i32);
pub struct PrintMediaType(i32);
pub struct PrintOrientation(i32);
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription(i32);
pub struct PrintPageInfo(i32);
pub struct PrintPageRange(i32);
pub struct PrintPageRangeOptions(i32);
pub struct PrintQuality(i32);
pub struct PrintStaple(i32);
pub struct PrintTask(i32);
pub struct PrintTaskCompletedEventArgs(i32);
pub struct PrintTaskCompletion(i32);
pub struct PrintTaskOptions(i32);
pub struct PrintTaskProgressingEventArgs(i32);
pub struct PrintTaskRequest(i32);
pub struct PrintTaskRequestedDeferral(i32);
pub struct PrintTaskRequestedEventArgs(i32);
pub struct PrintTaskSourceRequestedArgs(i32);
pub struct PrintTaskSourceRequestedDeferral(i32);
pub struct PrintTaskSourceRequestedHandler(pub *mut ::core::ffi::c_void);
pub struct StandardPrintTaskOptions(i32);

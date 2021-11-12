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
pub struct PrintBinding(i32);
pub struct PrintBordering(i32);
pub struct PrintCollation(i32);
pub struct PrintColorMode(i32);
pub struct PrintDuplex(i32);
pub struct PrintHolePunch(i32);
#[repr(transparent)]
pub struct PrintManager(pub *mut ::core::ffi::c_void);
pub struct PrintMediaSize(i32);
pub struct PrintMediaType(i32);
pub struct PrintOrientation(i32);
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription(i32);
#[repr(transparent)]
pub struct PrintPageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintPageRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintPageRangeOptions(pub *mut ::core::ffi::c_void);
pub struct PrintQuality(i32);
pub struct PrintStaple(i32);
#[repr(transparent)]
pub struct PrintTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct PrintTaskCompletion(i32);
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
#[repr(transparent)]
pub struct StandardPrintTaskOptions(pub *mut ::core::ffi::c_void);

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
extern "system" {
    fn IPrintDocumentSource();
    fn IPrintManager();
    fn IPrintManagerStatic();
    fn IPrintManagerStatic2();
    fn IPrintPageInfo();
    fn IPrintPageRange();
    fn IPrintPageRangeFactory();
    fn IPrintPageRangeOptions();
    fn IPrintTask();
    fn IPrintTask2();
    fn IPrintTaskCompletedEventArgs();
    fn IPrintTaskOptions();
    fn IPrintTaskOptions2();
    fn IPrintTaskOptionsCore();
    fn IPrintTaskOptionsCoreProperties();
    fn IPrintTaskOptionsCoreUIConfiguration();
    fn IPrintTaskProgressingEventArgs();
    fn IPrintTaskRequest();
    fn IPrintTaskRequestedDeferral();
    fn IPrintTaskRequestedEventArgs();
    fn IPrintTaskSourceRequestedArgs();
    fn IPrintTaskSourceRequestedDeferral();
    fn IPrintTaskTargetDeviceSupport();
    fn IStandardPrintTaskOptionsStatic();
    fn IStandardPrintTaskOptionsStatic2();
    fn IStandardPrintTaskOptionsStatic3();
    fn PrintBinding();
    fn PrintBordering();
    fn PrintCollation();
    fn PrintColorMode();
    fn PrintDuplex();
    fn PrintHolePunch();
    fn PrintManager();
    fn PrintMediaSize();
    fn PrintMediaType();
    fn PrintOrientation();
    fn PrintPageDescription();
    fn PrintPageInfo();
    fn PrintPageRange();
    fn PrintPageRangeOptions();
    fn PrintQuality();
    fn PrintStaple();
    fn PrintTask();
    fn PrintTaskCompletedEventArgs();
    fn PrintTaskCompletion();
    fn PrintTaskOptions();
    fn PrintTaskProgressingEventArgs();
    fn PrintTaskRequest();
    fn PrintTaskRequestedDeferral();
    fn PrintTaskRequestedEventArgs();
    fn PrintTaskSourceRequestedArgs();
    fn PrintTaskSourceRequestedDeferral();
    fn PrintTaskSourceRequestedHandler();
    fn StandardPrintTaskOptions();
}

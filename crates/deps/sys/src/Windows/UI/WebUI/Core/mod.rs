#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWebUICommandBar();
    fn IWebUICommandBarBitmapIcon();
    fn IWebUICommandBarBitmapIconFactory();
    fn IWebUICommandBarConfirmationButton();
    fn IWebUICommandBarElement();
    fn IWebUICommandBarIcon();
    fn IWebUICommandBarIconButton();
    fn IWebUICommandBarItemInvokedEventArgs();
    fn IWebUICommandBarSizeChangedEventArgs();
    fn IWebUICommandBarStatics();
    fn IWebUICommandBarSymbolIcon();
    fn IWebUICommandBarSymbolIconFactory();
    fn MenuClosedEventHandler();
    fn MenuOpenedEventHandler();
    fn SizeChangedEventHandler();
    fn WebUICommandBar();
    fn WebUICommandBarBitmapIcon();
    fn WebUICommandBarClosedDisplayMode();
    fn WebUICommandBarConfirmationButton();
    fn WebUICommandBarContract();
    fn WebUICommandBarIconButton();
    fn WebUICommandBarItemInvokedEventArgs();
    fn WebUICommandBarSizeChangedEventArgs();
    fn WebUICommandBarSymbolIcon();
}

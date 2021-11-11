#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWebApplicationActivation();
    fn IWebApplicationAuthoringMode();
    fn IWebApplicationHost();
    fn IWebApplicationNavigationEvents();
    fn IWebApplicationScriptEvents();
    fn IWebApplicationUIEvents();
    fn IWebApplicationUpdateEvents();
    fn RegisterAuthoringClientFunctionType();
    fn UnregisterAuthoringClientFunctionType();
}

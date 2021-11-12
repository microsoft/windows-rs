#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWebApplicationActivation(i32);
pub struct IWebApplicationAuthoringMode(i32);
pub struct IWebApplicationHost(i32);
pub struct IWebApplicationNavigationEvents(i32);
pub struct IWebApplicationScriptEvents(i32);
pub struct IWebApplicationUIEvents(i32);
pub struct IWebApplicationUpdateEvents(i32);
pub struct RegisterAuthoringClientFunctionType(i32);
pub struct UnregisterAuthoringClientFunctionType(i32);

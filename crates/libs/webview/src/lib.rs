#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(non_snake_case, dead_code, non_camel_case_types)]

mod bindings;
// TODO: need to hide this
pub use bindings::*;

use windows_link::*;
use windows_strings::*;
use windows_core::*;

link!("WebView2Loader.dll" "system" fn CreateCoreWebView2EnvironmentWithOptions(browserExecutableFolder: PCWSTR, userDataFolder: PCWSTR, environmentOptions: Ref<ICoreWebView2EnvironmentOptions>, environmentCreatedHandler: Ref<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>) -> HRESULT);

link!("WebView2Loader.dll" "system" fn CreateCoreWebView2Environment(environmentCreatedHandler: Ref<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>) -> HRESULT);

unsafe impl Send for HWND{}
unsafe impl Sync for HWND{}
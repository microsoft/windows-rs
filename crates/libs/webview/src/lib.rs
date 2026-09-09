#![doc = include_str!("../readme.md")]

#[cfg_attr(not(feature = "reactor"), allow(dead_code))]
#[expect(
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings;
mod controller;
mod cookie;
mod deferral;
mod download;
mod environment;
mod event;
mod handler;
mod host;
mod options;
mod profile;
mod protocol;
#[cfg(feature = "reactor")]
mod reactor;
mod script;
mod settings;
mod string;
mod webview;
#[cfg(feature = "system")]
mod window;

use bindings::*;
use windows_core::*;

pub use controller::{Color, Controller, ControllerOptions};
pub use cookie::{Cookie, CookieManager, SameSite};
pub use deferral::Deferral;
pub use download::{
    DownloadInterruptReason, DownloadOperation, DownloadStartingArgs, DownloadState,
};
pub use environment::Environment;
pub use event::{
    AcceleratorKeyPressedArgs, ContentLoadingArgs, DevToolsProtocolEventReceivedArgs,
    EventRegistration, KeyEventKind, MoveFocusReason, MoveFocusRequestedArgs,
    NavigationCompletedArgs, NavigationStartingArgs, NewWindowRequestedArgs, PermissionKind,
    PermissionRequestedArgs, PermissionState, ProcessFailedArgs, ProcessFailedKind,
    WebMessageReceivedArgs,
};
pub use host::{WebViewHost, WebViewHostBuilder};
pub use options::{EnvironmentOptions, ScrollBarStyle};
pub use profile::{PreferredColorScheme, Profile};
pub use protocol::{WebResourceRequest, WebResourceResponse};
#[cfg(feature = "reactor")]
pub use reactor::{webview, webview_result};
pub use script::ScriptId;
pub use settings::Settings;
pub use webview::{HostResourceAccessKind, MemoryUsageTargetLevel, NavigationRequest, WebView};
#[cfg(feature = "system")]
pub use window::{WebViewWindow, WebViewWindowBuilder};
pub use windows_core::Result;

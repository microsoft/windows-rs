#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
mod controller;
mod cookie;
mod deferral;
mod download;
mod environment;
mod event;
mod handler;
mod options;
mod profile;
mod protocol;
mod pump;
mod script;
mod settings;
mod string;
mod webview;

use bindings::*;
use windows_core::*;

pub use bindings::HWND;
pub use controller::{Color, Controller, ControllerOptions};
pub use cookie::{Cookie, CookieManager, SameSite};
pub use deferral::Deferral;
pub use download::{
    DownloadInterruptReason, DownloadOperation, DownloadStartingArgs, DownloadState,
};
pub use environment::Environment;
pub use event::{
    AcceleratorKeyPressedArgs, ContentLoadingArgs, EventRegistration, KeyEventKind,
    MoveFocusReason, MoveFocusRequestedArgs, NavigationCompletedArgs, NavigationStartingArgs,
    NewWindowRequestedArgs, PermissionKind, PermissionRequestedArgs, PermissionState,
    ProcessFailedArgs, ProcessFailedKind, WebMessageReceivedArgs,
};
pub use options::{EnvironmentOptions, ScrollBarStyle};
pub use profile::{PreferredColorScheme, Profile};
pub use protocol::{WebResourceRequest, WebResourceResponse};
pub use script::ScriptId;
pub use settings::Settings;
pub use webview::{HostResourceAccessKind, MemoryUsageTargetLevel, NavigationRequest, WebView};
pub use windows_core::Result;

#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
mod controller;
mod deferral;
mod download;
mod environment;
mod event;
mod handler;
mod options;
mod protocol;
mod pump;
mod script;
mod settings;
mod string;
mod webview;

use bindings::*;
use windows_core::*;

pub use bindings::HWND;
pub use controller::Controller;
pub use deferral::Deferral;
pub use download::{
    DownloadInterruptReason, DownloadOperation, DownloadStartingArgs, DownloadState,
};
pub use environment::Environment;
pub use event::{
    ContentLoadingArgs, EventRegistration, NavigationCompletedArgs, NavigationStartingArgs,
    NewWindowRequestedArgs, PermissionKind, PermissionRequestedArgs, PermissionState,
    ProcessFailedArgs, ProcessFailedKind, WebMessageReceivedArgs,
};
pub use options::EnvironmentOptions;
pub use protocol::{WebResourceRequest, WebResourceResponse};
pub use script::ScriptId;
pub use settings::Settings;
pub use webview::WebView;
pub use windows_core::Result;

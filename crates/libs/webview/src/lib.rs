#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
mod controller;
mod deferral;
mod environment;
mod event;
mod handler;
mod options;
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
pub use environment::Environment;
pub use event::{
    ContentLoadingArgs, EventRegistration, NavigationCompletedArgs, NavigationStartingArgs,
    NewWindowRequestedArgs, PermissionKind, PermissionRequestedArgs, PermissionState,
    WebMessageReceivedArgs,
};
pub use options::EnvironmentOptions;
pub use script::ScriptId;
pub use settings::Settings;
pub use webview::WebView;
pub use windows_core::Result;

//! Library surface of `tool_reactor`, exposing the winmd staging used both by the reactor
//! code generator (this package's binary) and by `tool_webview`, whose WinUI XAML `WebView2`
//! bindings are generated from the same fetched-on-demand WinUI / Windows App SDK metadata.
//! Keeping the version constants and staging in one place means a single bump updates every
//! consumer.

pub mod stage;

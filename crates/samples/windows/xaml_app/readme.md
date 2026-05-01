## XAML app sample

This sample demonstrates how to host a [XAML](https://learn.microsoft.com/uwp/api/windows.ui.xaml)
window from Rust by implementing the `IApplicationOverrides` interface and aggregating
`Windows.UI.Xaml.Application`.

The `Windows.UI.Xaml.*` namespaces are no longer included in the `windows` crate, so this
sample uses [`windows-bindgen`](https://crates.io/crates/windows-bindgen) from a `build.rs`
to generate the minimum subset of XAML bindings the sample actually needs (`Application`,
`IApplicationOverrides`, `ApplicationInitializationCallback`, `Window` and
`Controls.TextBox`). Bindings are written to `src/bindings.rs` at build time and reference
the rest of the Windows API surface (Win32, Foundation, ApplicationModel) from the
`windows` crate dependency.

XAML applications must be packaged before they can run. To build, register, and launch the
sample, run `register.cmd` from a Visual Studio "x64 Native Tools" command prompt, then
launch "Rust XamlApp" from the Start menu.

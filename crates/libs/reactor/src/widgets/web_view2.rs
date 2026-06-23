use super::*;

/// Opaque handle to the native `WebView2` control, passed to the
/// [`on_mounted`](WebView2::on_mounted) / [`on_unmounted`](WebView2::on_unmounted)
/// callbacks.
///
/// `windows-reactor` hosts the control but deliberately knows nothing about the
/// WebView2 API. The native `Microsoft.UI.Xaml.Controls.WebView2` instance is
/// exposed as an [`IInspectable`](windows_core::IInspectable) so a higher layer
/// (the `windows-webview` crate's optional `reactor` feature) can drive it —
/// `EnsureCoreWebView2Async`, the WinRT → COM `ICoreWebView2` bridge, navigation,
/// and events all live there, not here.
#[derive(Clone)]
pub struct WebView2Handle(windows_core::IInspectable);

impl WebView2Handle {
    /// The native `Microsoft.UI.Xaml.Controls.WebView2` control as an
    /// `IInspectable`, ready to be cast to the consumer's own WebView2 bindings.
    pub fn as_inspectable(&self) -> &windows_core::IInspectable {
        &self.0
    }
}

/// Built-in widget for `Microsoft.UI.Xaml.Controls.WebView2` — hosts a WebView2
/// browser control inside a WinUI 3 XAML tree.
///
/// Like [`SwapChainPanel`](crate::SwapChainPanel), this is a thin native-control
/// host with no reactive properties: use [`on_mounted`](WebView2::on_mounted) to
/// receive a [`WebView2Handle`] and drive the control from a higher layer.
#[derive(Clone, Debug, PartialEq)]
pub struct WebView2 {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub mounted: Option<Callback<Option<windows_core::IInspectable>>>,
    pub unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl Default for WebView2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WebView2 {
    pub fn new() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            mounted: None,
            unmounted: None,
        }
    }

    /// Callback invoked once after the native control is created.
    pub fn on_mounted(mut self, f: impl Fn(WebView2Handle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(WebView2Handle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native control is destroyed, while it
    /// still exists. Use this to tear down resources bound to the control.
    pub fn on_unmounted(mut self, f: impl Fn(WebView2Handle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(WebView2Handle(native));
            }
        }));
        self
    }
}

impl Widget for WebView2 {
    widget_header!(ControlKind::WebView2);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
    fn on_mounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
        self.mounted.as_ref()
    }
    fn on_unmounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
        self.unmounted.as_ref()
    }
}

/// Factory function for a [`WebView2`].
pub fn web_view2() -> WebView2 {
    WebView2::new()
}

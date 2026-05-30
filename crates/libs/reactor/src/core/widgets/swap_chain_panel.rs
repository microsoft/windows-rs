use super::*;

/// Built-in widget for `Microsoft.UI.Xaml.Controls.SwapChainPanel` — hosts
/// custom Direct3D / Direct2D rendering inside a WinUI 3 XAML tree.
///
/// Use [`on_mounted`](SwapChainPanelWidget::on_mounted) to receive the native
/// element and wire a DXGI swap chain via `ISwapChainPanelNative`.
#[derive(Clone, Debug, PartialEq)]
pub struct SwapChainPanelWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub mounted: Option<Callback<windows_core::IInspectable>>,
}

impl Default for SwapChainPanelWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SwapChainPanelWidget {
    pub fn new() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            mounted: None,
        }
    }

    /// Callback invoked once after the control is created. The argument is
    /// the `SwapChainPanel` as an `IInspectable`.
    pub fn on_mounted(mut self, f: impl Fn(windows_core::IInspectable) + 'static) -> Self {
        self.mounted = Some(Callback::new(f));
        self
    }
}

impl Widget for SwapChainPanelWidget {
    widget_header!(ControlKind::SwapChainPanel);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
    fn on_mounted_callback(&self) -> Option<&Callback<windows_core::IInspectable>> {
        self.mounted.as_ref()
    }
}

/// Factory function for a [`SwapChainPanelWidget`].
pub fn swap_chain_panel() -> SwapChainPanelWidget {
    SwapChainPanelWidget::new()
}

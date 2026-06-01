use super::*;
use windows_core::Interface;

/// Opaque handle to the native `SwapChainPanel` control, passed to the
/// [`on_ready`](SwapChainPanelWidget::on_ready) callback.
pub struct SwapChainPanelHandle(windows_core::IInspectable);

impl SwapChainPanelHandle {
    /// Attach a DXGI swap chain (created with `CreateSwapChainForComposition`).
    pub fn set_swap_chain(&self, swap_chain: &impl Interface) -> windows_core::Result<()> {
        let native: crate::bindings::ISwapChainPanelNative = self.0.cast()?;
        unsafe { native.SetSwapChain(swap_chain.as_raw()) }
    }
}

/// Built-in widget for `Microsoft.UI.Xaml.Controls.SwapChainPanel` — hosts
/// custom Direct3D / Direct2D rendering inside a WinUI 3 XAML tree.
///
/// Use [`on_ready`](SwapChainPanelWidget::on_ready) to receive a
/// [`SwapChainPanelHandle`] for attaching your DXGI swap chain.
#[derive(Clone, Debug, PartialEq)]
pub struct SwapChainPanelWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub(crate) mounted: Option<Callback<windows_core::IInspectable>>,
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

    /// Callback invoked once after the native control is created.
    pub fn on_ready(mut self, f: impl Fn(SwapChainPanelHandle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native| f(SwapChainPanelHandle(native))));
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

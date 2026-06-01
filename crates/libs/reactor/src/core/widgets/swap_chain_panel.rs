use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use windows_core::Interface;

/// Opaque handle to the native `SwapChainPanel` control, passed to the
/// [`on_ready`](SwapChainPanelWidget::on_ready) callback.
pub struct SwapChainPanelHandle(windows_core::IInspectable);

impl SwapChainPanelHandle {
    /// Attach a DXGI swap chain (created with `CreateSwapChainForComposition`).
    ///
    /// # Safety contract
    /// The caller must pass a valid `IDXGISwapChain` (or `IDXGISwapChain1`).
    /// Passing an unrelated COM interface will fail at the WinUI layer.
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

    /// Callback invoked when the panel's layout size changes (width, height in
    /// DIPs). Also fires once after the first layout pass. Use this to resize
    /// your swap chain buffers.
    pub fn on_resize(mut self, f: impl Fn(f64, f64) + 'static) -> Self {
        let f = Rc::new(f);
        let prev = self.mounted.take();
        self.mounted = Some(Callback::new(move |native: windows_core::IInspectable| {
            if let Some(ref cb) = prev {
                cb.invoke(native.clone());
            }
            // Subscribe to SizeChanged on the FrameworkElement.
            if let Ok(fe) = native.cast::<crate::bindings::IFrameworkElement>() {
                let f = f.clone();
                // Store the revoker so the subscription lives as long as the control.
                let revoker: Rc<RefCell<Option<windows_core::EventRevoker>>> =
                    Rc::new(RefCell::new(None));
                let r = fe.add_SizeChanged(move |_sender, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(s) = args.get_NewSize()
                    {
                        f(s.Width as f64, s.Height as f64);
                    }
                });
                if let Ok(revoker_val) = r {
                    *revoker.borrow_mut() = Some(revoker_val);
                    // Leak the Rc so the subscription outlives this scope.
                    // The revoker prevent leaks — it revokes on Drop when
                    // the control is destroyed.
                    std::mem::forget(revoker);
                }
            }
        }));
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

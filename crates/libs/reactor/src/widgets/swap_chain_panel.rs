use super::*;
use std::rc::Rc;

/// Opaque handle to the native `SwapChainPanel` control, passed to the
/// [`on_mounted`](SwapChainPanel::on_mounted) callback.
#[derive(Clone)]
pub struct SwapChainPanelHandle(windows_core::IInspectable);

impl SwapChainPanelHandle {
    /// Wraps the native `SwapChainPanel` element for a control created outside
    /// the [`swap_chain_panel()`] widget — for example, a [`CustomElement`] that
    /// creates a `ControlKind::SwapChainPanel` and obtains its native object via
    /// [`Backend::get_native_element`](crate::Backend::get_native_element). This
    /// lets such a host attach a `windows-canvas` swap chain or a raw DXGI swap
    /// chain to the panel.
    pub fn from_native(native: windows_core::IInspectable) -> Self {
        Self(native)
    }

    /// Attach a DXGI swap chain (created with `CreateSwapChainForComposition`).
    ///
    /// # Safety contract
    /// The caller must pass a valid `IDXGISwapChain` (or `IDXGISwapChain1`).
    /// Passing an unrelated COM interface will fail at the WinUI layer.
    pub fn set_swap_chain(&self, swap_chain: &impl Interface) -> Result<()> {
        let native: bindings::ISwapChainPanelNative = self.0.cast()?;
        unsafe { native.SetSwapChain(swap_chain.as_raw()).ok() }
    }

    /// Detach any swap chain previously attached to the panel, clearing its
    /// presented content. Use when a host stops rendering (for example, a
    /// [`CustomElement`] switching to a software fallback) without destroying
    /// the panel.
    pub fn clear_swap_chain(&self) -> Result<()> {
        let native: bindings::ISwapChainPanelNative = self.0.cast()?;
        unsafe { native.SetSwapChain(std::ptr::null_mut()).ok() }
    }

    /// Returns the current composition scale (DPI scale factor) as `(scale_x, scale_y)`.
    ///
    /// Multiply DIP dimensions by these values to get pixel dimensions for the swap chain.
    /// Typically both values are equal (e.g., 1.5 at 150% display scaling).
    pub fn composition_scale(&self) -> Result<(f32, f32)> {
        let panel: bindings::ISwapChainPanel = self.0.cast()?;
        let x = panel.CompositionScaleX()?;
        let y = panel.CompositionScaleY()?;
        Ok((x, y))
    }

    /// Subscribe to composition scale changes (e.g., window moved to a different monitor).
    ///
    /// The callback receives `(scale_x, scale_y)`.
    pub fn on_composition_scale_changed(
        &self,
        f: impl Fn(f32, f32) + 'static,
    ) -> Result<windows_core::EventRevoker> {
        let panel: bindings::ISwapChainPanel = self.0.cast()?;
        panel.CompositionScaleChanged(move |sender, _| {
            if let Some(sender) = sender.as_ref() {
                let scp: &bindings::ISwapChainPanel = sender;
                let x = scp.CompositionScaleX().unwrap_or(1.0);
                let y = scp.CompositionScaleY().unwrap_or(1.0);
                f(x, y);
            }
        })
    }
}

/// Built-in widget for `Microsoft.UI.Xaml.Controls.SwapChainPanel` — hosts
/// custom Direct3D / Direct2D rendering inside a WinUI 3 XAML tree.
///
/// Use [`on_mounted`](SwapChainPanel::on_mounted) to receive a
/// [`SwapChainPanelHandle`] for attaching your DXGI swap chain.
#[derive(Clone, Debug, PartialEq)]
pub struct SwapChainPanel {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub mounted: Option<Callback<Option<windows_core::IInspectable>>>,
    pub unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl Default for SwapChainPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl SwapChainPanel {
    pub fn new() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            mounted: None,
            unmounted: None,
        }
    }

    /// Callback invoked once after the native control is created.
    pub fn on_mounted(mut self, f: impl Fn(SwapChainPanelHandle) + 'static) -> Self {
        // A `SwapChainPanel` always has a native control in practice; the
        // handle is only built when one is present.
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(SwapChainPanelHandle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native control is destroyed, while it
    /// still exists. Use this to tear down resources bound to the panel (for
    /// example, stop and join a render thread that presents into its swap
    /// chain) before the panel — and its swap chain — go away.
    pub fn on_unmounted(mut self, f: impl Fn(SwapChainPanelHandle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(SwapChainPanelHandle(native));
            }
        }));
        self
    }

    /// Callback invoked when the panel's layout size changes (width, height in
    /// DIPs). Also fires once after the first layout pass. Use this to resize
    /// your swap chain buffers.
    pub fn on_resize(mut self, f: impl Fn(f64, f64) + 'static) -> Self {
        let f = Rc::new(f);
        let prev = self.mounted.take();
        self.mounted = Some(Callback::new(move |native: Option<windows_core::IInspectable>| {
            if let Some(ref cb) = prev {
                cb.invoke(native.clone());
            }
            let Some(native) = native else {
                return;
            };
            // Subscribe to SizeChanged on the FrameworkElement.
            if let Ok(fe) = native.cast::<bindings::IFrameworkElement>() {
                let f = f.clone();
                if let Ok(revoker) = fe.SizeChanged(move |_sender, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(s) = args.NewSize()
                    {
                        f(s.width as f64, s.height as f64);
                    }
                }) {
                    // Fire-and-forget for the element's lifetime. `into_token`
                    // drops the revoker's strong reference to the element (unlike
                    // `forget`, which would pin the element alive forever); the
                    // handler is torn down when WinUI destroys the element.
                    let _ = revoker.into_token();
                }
            }
        }));
        self
    }
}

impl Widget for SwapChainPanel {
    widget_header!(ControlKind::SwapChainPanel);
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

/// Factory function for a [`SwapChainPanel`].
pub fn swap_chain_panel() -> SwapChainPanel {
    SwapChainPanel::new()
}

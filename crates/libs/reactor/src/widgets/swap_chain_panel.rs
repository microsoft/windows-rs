use super::*;
use std::rc::Rc;

/// Opaque handle to the native `SwapChainPanel` control, passed to the
/// [`on_mounted`](SwapChainPanel::on_mounted) callback.
#[derive(Clone)]
pub struct SwapChainPanelHandle(windows_core::IInspectable);

impl SwapChainPanelHandle {
    /// Wraps a native `SwapChainPanel` created outside [`swap_chain_panel()`].
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

    /// Detaches any swap chain previously attached to the panel.
    pub fn clear_swap_chain(&self) -> Result<()> {
        let native: bindings::ISwapChainPanelNative = self.0.cast()?;
        unsafe { native.SetSwapChain(std::ptr::null_mut()).ok() }
    }

    /// Returns the current composition scale as `(scale_x, scale_y)`.
    pub fn composition_scale(&self) -> Result<(f32, f32)> {
        let panel: bindings::ISwapChainPanel = self.0.cast()?;
        let x = panel.CompositionScaleX()?;
        let y = panel.CompositionScaleY()?;
        Ok((x, y))
    }

    /// Subscribes to composition scale changes.
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

/// Widget that hosts custom Direct3D / Direct2D rendering inside WinUI.
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
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(SwapChainPanelHandle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native control is destroyed.
    pub fn on_unmounted(mut self, f: impl Fn(SwapChainPanelHandle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(SwapChainPanelHandle(native));
            }
        }));
        self
    }

    /// Callback invoked when the panel's layout size changes.
    pub fn on_resize(mut self, f: impl Fn(f64, f64) + 'static) -> Self {
        let f = Rc::new(f);
        let prev = self.mounted.take();
        self.mounted = Some(Callback::new(
            move |native: Option<windows_core::IInspectable>| {
                if let Some(ref cb) = prev {
                    cb.invoke(native.clone());
                }
                let Some(native) = native else {
                    return;
                };
                if let Ok(fe) = native.cast::<bindings::IFrameworkElement>() {
                    let f = f.clone();
                    if let Ok(revoker) = fe.SizeChanged(move |_sender, args| {
                        if let Some(args) = args.as_ref()
                            && let Ok(s) = args.NewSize()
                        {
                            f(s.width as f64, s.height as f64);
                        }
                    }) {
                        // `into_token` avoids pinning the element alive forever.
                        let _ = revoker.into_token();
                    }
                }
            },
        ));
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

/// Creates a [`SwapChainPanel`].
pub fn swap_chain_panel() -> SwapChainPanel {
    SwapChainPanel::new()
}

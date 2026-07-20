use super::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Opaque handle to a composition host element, passed to the
/// [`on_mounted`](CompositionHost::on_mounted) callback.
///
/// Obtain the element's lifted [`Compositor`](windows_composition::Compositor)
/// via [`compositor`](Self::compositor) and attach a visual tree with
/// [`set_child_visual`](Self::set_child_visual) (backed by
/// `ElementCompositionPreview::SetElementChildVisual`):
///
/// ```ignore
/// # use windows_reactor::CompositionHostHandle;
/// # fn demo(host: CompositionHostHandle) -> windows_core::Result<()> {
/// let compositor = host.compositor()?;
/// let root = compositor.create_container_visual();
/// host.set_child_visual(&root)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct CompositionHostHandle(windows_core::IInspectable);

impl CompositionHostHandle {
    /// Returns the host element's lifted composition
    /// [`Compositor`](windows_composition::Compositor). Every visual attached
    /// with [`set_child_visual`](Self::set_child_visual) must be created from it.
    pub fn compositor(&self) -> Result<windows_composition::Compositor> {
        let element: bindings::UIElement = self.0.cast()?;
        let visual = bindings::ElementCompositionPreview::GetElementVisual(&element)?;
        Ok(windows_composition::Visual::from_host(visual.into())?.compositor())
    }

    /// Attaches `visual` as the host element's child visual, replacing any
    /// visual attached previously. `visual` must be created from the compositor
    /// returned by [`compositor`](Self::compositor).
    pub fn set_child_visual(&self, visual: &windows_composition::Visual) -> Result<()> {
        let element: bindings::UIElement = self.0.cast()?;
        let visual: bindings::Visual = visual.as_raw().cast()?;
        bindings::ElementCompositionPreview::SetElementChildVisual(&element, &visual)
    }

    /// Delivers the host's rasterization (DPI) scale to `f`: once the control is
    /// loaded into the tree, and again whenever the scale changes (for example
    /// the window moves to a monitor with different scaling).
    ///
    /// The scale is `1.0` at 96 DPI, `1.5` at 150%, `2.0` at 192 DPI. Multiply a
    /// composition size (in DIPs) by it to size a backing surface for crisp
    /// output. Keep the returned [`EventRevoker`](windows_core::EventRevoker)
    /// alive for as long as you want updates.
    pub fn on_rasterization_scale_changed(
        &self,
        f: impl Fn(f64) + 'static,
    ) -> Result<windows_core::EventRevoker> {
        let element: bindings::IFrameworkElement = self.0.cast()?;
        let f = Rc::new(f);
        // Owned by the `Loaded` closure so it is revoked when the returned
        // `Loaded` revoker is dropped.
        let changed: Rc<RefCell<Option<windows_core::EventRevoker>>> = Rc::new(RefCell::new(None));
        element.Loaded(move |sender, _| {
            let Some(element) = sender.as_ref().and_then(|s| s.cast::<bindings::IUIElement>().ok())
            else {
                return;
            };
            let Ok(root) = element.XamlRoot() else {
                return;
            };
            if let Ok(scale) = root.RasterizationScale() {
                f(scale);
            }
            let f = f.clone();
            let revoker = root.Changed(move |sender, _| {
                if let Some(sender) = sender.as_ref()
                    && let Ok(scale) = sender.RasterizationScale()
                {
                    f(scale);
                }
            });
            *changed.borrow_mut() = revoker.ok();
        })
    }
}

/// Built-in widget that hosts a custom `Microsoft.UI.Composition` visual tree
/// inside a WinUI 3 XAML tree — the composition-interop counterpart of
/// [`SwapChainPanel`] (which hosts a DXGI swap chain).
///
/// The host is backed by a plain stretching panel; use
/// [`on_mounted`](Self::on_mounted) to receive a [`CompositionHostHandle`] for
/// obtaining the compositor and attaching a visual tree.
#[derive(Clone, Debug, PartialEq)]
pub struct CompositionHost {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub mounted: Option<Callback<Option<windows_core::IInspectable>>>,
    pub unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl Default for CompositionHost {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositionHost {
    pub fn new() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            mounted: None,
            unmounted: None,
        }
    }

    /// Callback invoked once after the native host is created, with a
    /// [`CompositionHostHandle`] for wiring up composition content.
    pub fn on_mounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native host is destroyed, while it still
    /// exists. Use this to tear down composition resources bound to the host
    /// (for example detach the child visual) before it goes away.
    pub fn on_unmounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }

    /// Callback invoked when the host's layout size changes (width, height in
    /// DIPs). Also fires once after the first layout pass. Use this to resize
    /// composition content.
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

impl Widget for CompositionHost {
    // Backed by a plain stretching `Grid`; the composition child visual is
    // attached to the host element itself, so no child controls are needed.
    widget_header!(ControlKind::Grid);
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

/// Factory function for a [`CompositionHost`].
pub fn composition_host() -> CompositionHost {
    CompositionHost::new()
}

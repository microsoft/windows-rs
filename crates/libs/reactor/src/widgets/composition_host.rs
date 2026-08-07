use super::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Opaque handle to a composition host element.
///
/// Use [`compositor`](Self::compositor) to create visuals and
/// [`set_child_visual`](Self::set_child_visual) to attach them.
#[derive(Clone)]
pub struct CompositionHostHandle(windows_core::IInspectable);

impl sealed::ElementHandle for CompositionHostHandle {
    fn from_native(native: windows_core::IInspectable) -> Self {
        Self(native)
    }
}

impl CompositionHostHandle {
    /// Returns the host element's lifted composition compositor.
    pub fn compositor(&self) -> Result<windows_composition::Compositor> {
        let element: bindings::UIElement = self.0.cast()?;
        let visual = bindings::ElementCompositionPreview::GetElementVisual(&element)?;
        Ok(windows_composition::Visual::from_host(visual.into())?.compositor())
    }

    /// Attaches `visual` as the host element's child visual.
    pub fn set_child_visual(&self, visual: &windows_composition::Visual) -> Result<()> {
        let element: bindings::UIElement = self.0.cast()?;
        let visual: bindings::Visual = visual.as_raw().cast()?;
        bindings::ElementCompositionPreview::SetElementChildVisual(&element, &visual)
    }

    /// Calls `f` with the DPI scale on load and whenever it changes.
    pub fn on_rasterization_scale_changed(
        &self,
        f: impl Fn(f64) + 'static,
    ) -> Result<windows_core::EventRevoker> {
        let element: bindings::IFrameworkElement = self.0.cast()?;
        let f = Rc::new(f);
        // Revoked when the returned Loaded revoker is dropped.
        let changed: Rc<RefCell<Option<windows_core::EventRevoker>>> = Rc::new(RefCell::new(None));
        element.Loaded(move |sender, _| {
            let Some(element) = sender
                .as_ref()
                .and_then(|s| s.cast::<bindings::IUIElement>().ok())
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

/// Widget that hosts a custom composition visual tree inside WinUI.
#[derive(Clone, Debug, PartialEq)]
pub struct CompositionHost {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub mounted: Option<Callback<Option<windows_core::IInspectable>>>,
    pub unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl ElementRefExt for CompositionHost {
    type Handle = CompositionHostHandle;
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

    /// Callback invoked once after the native host is created.
    pub fn on_mounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native host is destroyed.
    pub fn on_unmounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }

    /// Callback invoked when the host's layout size changes.
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

impl Widget for CompositionHost {
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

/// Creates a [`CompositionHost`].
pub fn composition_host() -> CompositionHost {
    CompositionHost::new()
}

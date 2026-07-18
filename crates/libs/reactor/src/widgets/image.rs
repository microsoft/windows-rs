use super::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Opaque handle to the native `Image` control, passed to the
/// [`on_mounted`](Image::on_mounted) callback.
#[derive(Clone)]
pub struct ImageHandle(windows_core::IInspectable);

impl ImageHandle {
    /// Deliver the host's rasterization (DPI) scale to `f`: once the control is
    /// loaded into the tree, and again whenever the scale changes (for example the
    /// window moves to a monitor with different scaling).
    ///
    /// The scale is `1.0` at 96 DPI, `1.5` at 150%, `2.0` at 192 DPI. Multiply a
    /// device-independent size by it to allocate a crisp on-demand surface — pass
    /// the result as the `scale` argument to `CanvasImageSource::new`. The scale
    /// is only available once the control is loaded, so it is delivered through
    /// this callback rather than returned directly.
    ///
    /// Keep the returned [`EventRevoker`](windows_core::EventRevoker) alive for as
    /// long as you want updates; dropping it revokes both this and the underlying
    /// scale-changed subscription.
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

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub source: ImageSource,
    pub stretch: Stretch,
    pub mounted: Option<Callback<Option<windows_core::IInspectable>>>,
}
impl Default for Image {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            source: ImageSource::default(),
            stretch: Stretch::Uniform,
            mounted: None,
        }
    }
}
#[derive(Clone, Default, Debug, PartialEq)]
pub enum ImageSource {
    #[default]
    None,
    Uri(String),
    Surface(SurfaceImageSource),
}
impl From<SurfaceImageSource> for ImageSource {
    fn from(source: SurfaceImageSource) -> Self {
        Self::Surface(source)
    }
}
impl From<Option<SurfaceImageSource>> for ImageSource {
    fn from(source: Option<SurfaceImageSource>) -> Self {
        source.map_or(Self::None, ImageSource::Surface)
    }
}
impl Image {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn new_with_uri(source: impl Into<String>) -> Self {
        Self::new(ImageSource::Uri(source.into()))
    }

    pub fn stretch(mut self, v: Stretch) -> Self {
        self.stretch = v;
        self
    }

    /// Callback invoked once after the native control is created, receiving an
    /// [`ImageHandle`]. Use it to observe the host rasterization (DPI) scale via
    /// [`ImageHandle::on_rasterization_scale_changed`] so an on-demand surface can
    /// be sized for crisp rendering.
    pub fn on_mounted(mut self, f: impl Fn(ImageHandle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(ImageHandle(native));
            }
        }));
        self
    }
}

impl Widget for Image {
    widget_header!(ControlKind::Image);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::image_bindings(self);
        // ImageSource is a compound type not expressible in TOML.
        match &self.source {
            ImageSource::Uri(uri) => {
                out.push(Binding::Prop(Prop::ImageSource, PropValue::Str(uri.clone())));
            }
            ImageSource::Surface(s) => {
                out.push(Binding::Prop(
                    Prop::ImageSource,
                    PropValue::SurfaceImageSource(s.clone()),
                ));
            }
            ImageSource::None => {}
        }
        out
    }
    fn on_mounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
        self.mounted.as_ref()
    }
}

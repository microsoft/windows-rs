use super::*;

/// A solid-color composition island to host under a [`CompositionHost`] via
/// [`CompositionHostHandle::set_color_island`].
///
/// It is a self-contained demonstration of the composition-visual interop seam:
/// a *same-compositor* `SpriteVisual` filled with a `CompositionColorBrush`,
/// centered in the host element and optionally spinning forever — all without an
/// app-level dependency on the `windows` crate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorIsland {
    /// Fill color of the hosted sprite visual.
    pub color: Color,
    /// Size of the sprite visual in DIPs; it is centered within the host element.
    pub width: f32,
    pub height: f32,
    /// When set, the visual spins around its center forever, completing one full
    /// revolution over this period.
    pub spin: Option<std::time::Duration>,
}

impl ColorIsland {
    /// A static (non-spinning) island of `color`, `width` × `height` DIPs.
    pub fn new(color: Color, width: f32, height: f32) -> Self {
        Self {
            color,
            width,
            height,
            spin: None,
        }
    }

    /// Spin the island forever, one revolution per `period`.
    pub fn spin(mut self, period: std::time::Duration) -> Self {
        self.spin = Some(period);
        self
    }
}

/// Opaque handle to the native element backing a [`CompositionHost`], passed to
/// the [`on_mounted`](CompositionHost::on_mounted) callback.
///
/// It exposes the composition-visual interop seam directly against the mounted
/// element so an app can host a custom composition island (custom-drawn or
/// off-thread animated content) under a plain reactor element.
#[derive(Clone)]
pub struct CompositionHostHandle(windows_core::IInspectable);

impl CompositionHostHandle {
    fn ui(&self) -> Result<bindings::UIElement> {
        self.0.cast()
    }

    /// The host element's rasterization (DPI) scale. Multiply DIP sizes by this
    /// to size composition surfaces crisply.
    pub fn rasterization_scale(&self) -> Result<f32> {
        Ok(self.ui()?.RasterizationScale()? as f32)
    }

    /// Host `island` under the element as a same-compositor `SpriteVisual`,
    /// replacing any previously hosted child visual. The visual is sized in DIPs
    /// with its rotation pivot at its own center and, when [`ColorIsland::spin`]
    /// is set, animates its rotation forever.
    pub fn set_color_island(&self, island: &ColorIsland) -> Result<()> {
        let ui = self.ui()?;
        let compositor = bindings::ElementCompositionPreview::GetElementVisual(&ui)?
            .cast::<bindings::ICompositionObject>()?
            .Compositor()?;
        let icomp = compositor.cast::<bindings::ICompositor>()?;

        let brush = icomp.CreateColorBrushWithColor(island.color)?;
        let sprite = icomp.CreateSpriteVisual()?;
        sprite.SetBrush(&brush.cast::<bindings::CompositionBrush>()?)?;

        let visual = sprite.cast::<bindings::IVisual>()?;
        visual.SetSize(windows_numerics::Vector2 {
            x: island.width,
            y: island.height,
        })?;
        visual.SetCenterPoint(windows_numerics::Vector3 {
            x: island.width / 2.0,
            y: island.height / 2.0,
            z: 0.0,
        })?;

        if let Some(period) = island.spin {
            let anim = icomp.CreateScalarKeyFrameAnimation()?;
            let kfa = anim.cast::<bindings::IKeyFrameAnimation>()?;
            kfa.SetDuration(TimeSpan::try_from(period).unwrap_or(TimeSpan::MAX))?;
            kfa.SetIterationBehavior(bindings::AnimationIterationBehavior::Forever)?;
            let easing = icomp
                .CreateLinearEasingFunction()?
                .cast::<bindings::CompositionEasingFunction>()?;
            anim.cast::<bindings::IScalarKeyFrameAnimation>()?
                .InsertKeyFrameWithEasingFunction(1.0, 360.0, &easing)?;
            sprite
                .cast::<bindings::ICompositionObject>()?
                .StartAnimation("RotationAngleInDegrees", &anim.cast::<bindings::CompositionAnimation>()?)?;
        }

        bindings::ElementCompositionPreview::SetElementChildVisual(
            &ui,
            &sprite.cast::<bindings::Visual>()?,
        )
    }
}

/// Built-in widget that hosts a custom composition island under a plain element.
///
/// Backed by the same native control as [`Canvas`], it forwards the mounted
/// element as a [`CompositionHostHandle`] (via [`on_mounted`](Self::on_mounted))
/// so an app can attach a composition `Visual` through the interop seam.
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

    /// Callback invoked once after the native host element is created, receiving a
    /// [`CompositionHostHandle`] for attaching a composition island.
    pub fn on_mounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.mounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }

    /// Callback invoked just before the native host element is destroyed, while it
    /// still exists. Use this to tear down resources bound to the hosted visual.
    pub fn on_unmounted(mut self, f: impl Fn(CompositionHostHandle) + 'static) -> Self {
        self.unmounted = Some(Callback::new(move |native: Option<_>| {
            if let Some(native) = native {
                f(CompositionHostHandle(native));
            }
        }));
        self
    }
}

impl Widget for CompositionHost {
    widget_header!(ControlKind::Canvas);
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

use super::*;

/// How a visual's edges are rendered relative to its clip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderMode {
    /// Inherit the border mode from the parent visual.
    Inherit,
    /// Antialias the visual's edges (soft).
    Soft,
    /// Render the visual's edges without antialiasing (hard).
    Hard,
}

impl From<BorderMode> for bindings::CompositionBorderMode {
    fn from(mode: BorderMode) -> Self {
        match mode {
            BorderMode::Inherit => Self::Inherit,
            BorderMode::Soft => Self::Soft,
            BorderMode::Hard => Self::Hard,
        }
    }
}

/// The base type for every composition visual: a node in the visual tree with a
/// position, size, opacity, and visibility. Concrete visuals (`SpriteVisual`,
/// `ContainerVisual`, `ShapeVisual`) deref to `Visual`, so these operations are
/// available on them directly.
#[derive(Clone)]
pub struct Visual(pub(crate) bindings::Visual);

impl Visual {
    /// Sets the visual's offset from its parent, in DIPs.
    pub fn set_offset(&self, x: f32, y: f32, z: f32) {
        self.0.SetOffset(Vector3 { x, y, z }).unwrap();
    }

    /// Returns the visual's offset from its parent.
    pub fn offset(&self) -> Vector3 {
        self.0.Offset().unwrap()
    }

    /// Sets the visual's size, in DIPs.
    pub fn set_size(&self, width: f32, height: f32) {
        self.0
            .SetSize(Vector2 {
                x: width,
                y: height,
            })
            .unwrap();
    }

    /// Returns the visual's size.
    pub fn size(&self) -> Vector2 {
        self.0.Size().unwrap()
    }

    /// Sets the visual's opacity in the range `0.0..=1.0`.
    pub fn set_opacity(&self, opacity: f32) {
        self.0.SetOpacity(opacity).unwrap();
    }

    /// Returns the visual's opacity.
    pub fn opacity(&self) -> f32 {
        self.0.Opacity().unwrap()
    }

    /// Sets whether the visual (and its subtree) is rendered.
    pub fn set_visible(&self, visible: bool) {
        self.0.SetIsVisible(visible).unwrap();
    }

    /// Returns whether the visual is visible.
    pub fn is_visible(&self) -> bool {
        self.0.IsVisible().unwrap()
    }

    /// Surfaces the underlying visual as an [`IInspectable`](windows_core::IInspectable).
    ///
    /// This is the interop seam the reactor bridge uses to attach a visual tree
    /// to a WinUI host element via `ElementCompositionPreview`. Most callers
    /// should use the safe visual-tree API instead.
    #[cfg(feature = "lifted")]
    pub fn as_raw(&self) -> windows_core::IInspectable {
        self.0.clone().into()
    }

    /// Adopts a lifted `Microsoft.UI.Composition.Visual` surfaced as a raw
    /// [`IInspectable`](windows_core::IInspectable), so its properties and
    /// animations can be driven through this crate's safe API.
    ///
    /// This is the interop seam used by the reactor transition engine, which
    /// obtains an element's backing visual from `ElementCompositionPreview`.
    /// Lifted composition can only be hosted inside a WinUI element, so this has
    /// no system-stack counterpart.
    #[cfg(feature = "lifted")]
    pub fn from_host(visual: windows_core::IInspectable) -> Result<Self> {
        Ok(Self(visual.cast()?))
    }

    /// Sets the visual's scale factor about its [center point](Self::set_center_point).
    pub fn set_scale(&self, scale: Vector3) {
        self.0.SetScale(scale).unwrap();
    }

    /// Returns the visual's current scale factor.
    pub fn scale(&self) -> Vector3 {
        self.0.Scale().unwrap()
    }

    /// Sets the point, in DIPs, about which rotation and scaling are applied.
    pub fn set_center_point(&self, point: Vector3) {
        self.0.SetCenterPoint(point).unwrap();
    }

    /// Sets the normalized anchor point (`0.0..=1.0` per axis) that the visual's
    /// offset positions.
    pub fn set_anchor_point(&self, point: Vector2) {
        self.0.SetAnchorPoint(point).unwrap();
    }

    /// Sets how the visual's edges are rendered.
    pub fn set_border_mode(&self, mode: BorderMode) {
        self.0.SetBorderMode(mode.into()).unwrap();
    }

    /// Returns the visual's parent container, if it has one.
    pub fn parent(&self) -> Option<ContainerVisual> {
        // A root visual has no parent: the WinRT getter yields a null reference
        // (surfaced as an error), which maps to `None` rather than a panic.
        self.0.Parent().ok().map(ContainerVisual::new)
    }

    /// Sets the visual's size as a fraction of its parent's size (per axis).
    pub fn set_relative_size_adjustment(&self, adjustment: Vector2) {
        let visual: bindings::IVisual2 = self.0.cast().unwrap();
        visual.SetRelativeSizeAdjustment(adjustment).unwrap();
    }

    /// Sets an offset expressed as a fraction of the parent's size (per axis).
    pub fn set_relative_offset_adjustment(&self, adjustment: Vector3) {
        let visual: bindings::IVisual2 = self.0.cast().unwrap();
        visual.SetRelativeOffsetAdjustment(adjustment).unwrap();
    }

    /// Sets the visual whose coordinate space this visual's transform is
    /// relative to.
    pub fn set_parent_for_transform(&self, parent: &Self) {
        let visual: bindings::IVisual2 = self.0.cast().unwrap();
        visual.SetParentForTransform(&parent.0).unwrap();
    }

    /// Returns the compositor that created this visual.
    pub fn compositor(&self) -> Compositor {
        let object: bindings::ICompositionObject = self.0.cast().unwrap();
        Compositor(object.Compositor().unwrap())
    }

    /// Starts an animation on the named property (for example `"Scale"`).
    pub fn start_animation(&self, property: &str, animation: &impl Animation) {
        let object: bindings::ICompositionObject = self.0.cast().unwrap();
        object
            .StartAnimation(property, &animation.as_animation().0)
            .unwrap();
    }

    /// Attaches (or, with `None`, clears) the [`ImplicitAnimationCollection`]
    /// that animates this visual's properties automatically when they change.
    pub fn set_implicit_animations(&self, animations: Option<&ImplicitAnimationCollection>) {
        let object: bindings::ICompositionObject2 = self.0.cast().unwrap();
        object
            .SetImplicitAnimations(animations.map(|a| &a.0))
            .unwrap();
    }
}

/// A visual that hosts a child visual tree via its [`children`](Self::children).
#[derive(Clone)]
pub struct ContainerVisual {
    visual: Visual,
    container: bindings::ContainerVisual,
}

impl ContainerVisual {
    pub(crate) fn new(container: bindings::ContainerVisual) -> Self {
        Self {
            visual: Visual(container.cast().unwrap()),
            container,
        }
    }

    /// Returns the collection of child visuals.
    pub fn children(&self) -> VisualCollection {
        VisualCollection(self.container.Children().unwrap())
    }
}

impl core::ops::Deref for ContainerVisual {
    type Target = Visual;
    fn deref(&self) -> &Visual {
        &self.visual
    }
}

/// A visual that paints its bounds with any [`Brush`] (a solid color or a
/// nine-grid). Also a container: it derefs to [`ContainerVisual`], so it can
/// host child visuals and be positioned and sized like any [`Visual`].
#[derive(Clone)]
pub struct SpriteVisual {
    container: ContainerVisual,
    sprite: bindings::SpriteVisual,
}

impl SpriteVisual {
    pub(crate) fn new(sprite: bindings::SpriteVisual) -> Self {
        Self {
            container: ContainerVisual::new(sprite.cast().unwrap()),
            sprite,
        }
    }

    /// Sets the brush that fills the visual's bounds.
    pub fn set_brush(&self, brush: &impl Brush) {
        self.sprite.SetBrush(&brush.as_brush().0).unwrap();
    }
}

impl core::ops::Deref for SpriteVisual {
    type Target = ContainerVisual;
    fn deref(&self) -> &ContainerVisual {
        &self.container
    }
}

/// An ordered collection of child visuals under a [`ContainerVisual`].
#[derive(Clone)]
pub struct VisualCollection(pub(crate) bindings::VisualCollection);

impl VisualCollection {
    /// Returns the number of visuals in the collection.
    pub fn count(&self) -> i32 {
        self.0.Count().unwrap()
    }

    /// Inserts a visual at the top of the z-order (drawn last, on top).
    pub fn insert_at_top(&self, visual: &Visual) {
        self.0.InsertAtTop(&visual.0).unwrap();
    }

    /// Inserts a visual at the bottom of the z-order (drawn first, behind).
    pub fn insert_at_bottom(&self, visual: &Visual) {
        self.0.InsertAtBottom(&visual.0).unwrap();
    }

    /// Removes a visual from the collection.
    pub fn remove(&self, visual: &Visual) {
        self.0.Remove(&visual.0).unwrap();
    }

    /// Removes every visual from the collection.
    pub fn remove_all(&self) {
        self.0.RemoveAll().unwrap();
    }
}

use crate::bindings;
use crate::{
    BatchKind, Color, CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
    CompositionNineGridBrush, CompositionScopedBatch, CompositionSpriteShape, ContainerVisual,
    DesktopWindowTarget, ShapeVisual, SpriteVisual, Vector3KeyFrameAnimation,
};
use windows_core::{Interface, Result};

/// The composition engine — the factory for visuals, brushes, and window
/// targets.
///
/// Every visual and brush a `Compositor` creates belongs to that compositor and
/// can only be combined with its siblings.
#[derive(Clone)]
pub struct Compositor(pub(crate) bindings::Compositor);

impl Compositor {
    /// Creates a compositor.
    ///
    /// System composition (`Windows.UI.Composition`) is an OS component, so no
    /// runtime bootstrap is required — but a dispatcher queue must exist on the
    /// current thread first. Create a
    /// [`DispatcherQueueController`](crate::DispatcherQueueController) and keep
    /// it alive for the compositor's lifetime.
    ///
    /// ```no_run
    /// use windows_composition::{Compositor, DispatcherQueueController};
    ///
    /// let _queue = DispatcherQueueController::create_on_current_thread()?;
    /// let compositor = Compositor::new()?;
    /// # windows_core::Result::Ok(())
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self(bindings::Compositor::new()?))
    }

    /// Creates a composition target that hosts a visual tree inside a window.
    ///
    /// Set `is_topmost` to render above the window's other content. The returned
    /// [`DesktopWindowTarget`](crate::DesktopWindowTarget) must be kept alive for
    /// as long as its visual tree should be shown.
    ///
    /// ```no_run
    /// use windows_composition::{Compositor, DispatcherQueueController};
    /// use windows_window::Window;
    ///
    /// let window = Window::new("Composition").create()?;
    /// let _queue = DispatcherQueueController::create_on_current_thread()?;
    /// let compositor = Compositor::new()?;
    /// let target = compositor.create_desktop_window_target(&window, false)?;
    /// # windows_core::Result::Ok(())
    /// ```
    pub fn create_desktop_window_target(
        &self,
        window: &windows_window::Window,
        is_topmost: bool,
    ) -> Result<DesktopWindowTarget> {
        // SAFETY: `window` owns a live `HWND` for as long as the borrow lasts.
        unsafe { self.create_desktop_window_target_for_hwnd(window.hwnd(), is_topmost) }
    }

    /// Creates a composition target that hosts a visual tree inside a raw window
    /// handle.
    ///
    /// This is the escape hatch for callers that own an `HWND` from a source
    /// other than [`windows_window`]; most callers should prefer the safe
    /// [`create_desktop_window_target`](Self::create_desktop_window_target).
    ///
    /// # Safety
    ///
    /// `hwnd` must be a valid, live window handle owned by the calling thread.
    pub unsafe fn create_desktop_window_target_for_hwnd(
        &self,
        hwnd: *mut core::ffi::c_void,
        is_topmost: bool,
    ) -> Result<DesktopWindowTarget> {
        let interop: bindings::ICompositorDesktopInterop = self.0.cast()?;
        let target = unsafe { interop.CreateDesktopWindowTarget(hwnd, is_topmost)? };
        DesktopWindowTarget::new(target)
    }

    /// Creates an empty container visual that hosts a child visual tree.
    pub fn create_container_visual(&self) -> Result<ContainerVisual> {
        ContainerVisual::new(self.0.CreateContainerVisual()?)
    }

    /// Creates a sprite visual that paints itself with a brush.
    pub fn create_sprite_visual(&self) -> Result<SpriteVisual> {
        SpriteVisual::new(self.0.CreateSpriteVisual()?)
    }

    /// Creates a solid-color brush.
    pub fn create_color_brush(&self, color: Color) -> Result<CompositionColorBrush> {
        Ok(CompositionColorBrush(
            self.0.CreateColorBrushWithColor(color.0)?,
        ))
    }

    /// Creates a nine-grid brush.
    pub fn create_nine_grid_brush(&self) -> Result<CompositionNineGridBrush> {
        let compositor: bindings::ICompositor2 = self.0.cast()?;
        Ok(CompositionNineGridBrush(compositor.CreateNineGridBrush()?))
    }

    /// Creates a shape visual that renders composition shapes.
    pub fn create_shape_visual(&self) -> Result<ShapeVisual> {
        let compositor: bindings::ICompositor5 = self.0.cast()?;
        ShapeVisual::new(compositor.CreateShapeVisual()?)
    }

    /// Creates an empty container shape that groups child shapes.
    pub fn create_container_shape(&self) -> Result<CompositionContainerShape> {
        let compositor: bindings::ICompositor5 = self.0.cast()?;
        Ok(CompositionContainerShape(
            compositor.CreateContainerShape()?,
        ))
    }

    /// Creates an ellipse geometry.
    pub fn create_ellipse_geometry(&self) -> Result<CompositionEllipseGeometry> {
        let compositor: bindings::ICompositor5 = self.0.cast()?;
        Ok(CompositionEllipseGeometry(
            compositor.CreateEllipseGeometry()?,
        ))
    }

    /// Creates a sprite shape that fills the given geometry with a brush.
    pub fn create_sprite_shape(
        &self,
        geometry: &CompositionEllipseGeometry,
    ) -> Result<CompositionSpriteShape> {
        let compositor: bindings::ICompositor5 = self.0.cast()?;
        Ok(CompositionSpriteShape(
            compositor.CreateSpriteShapeWithGeometry(&geometry.as_geometry()?.0)?,
        ))
    }

    /// Creates a scoped batch that tracks the completion of the given kind of
    /// work.
    pub fn create_scoped_batch(&self, kind: BatchKind) -> Result<CompositionScopedBatch> {
        Ok(CompositionScopedBatch(
            self.0.CreateScopedBatch(kind.into())?,
        ))
    }

    /// Creates a `Vector3` key-frame animation.
    pub fn create_vector3_key_frame_animation(&self) -> Result<Vector3KeyFrameAnimation> {
        Ok(Vector3KeyFrameAnimation(
            self.0.CreateVector3KeyFrameAnimation()?,
        ))
    }
}

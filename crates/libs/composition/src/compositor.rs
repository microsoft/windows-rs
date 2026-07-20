use crate::bindings;
use crate::{
    BatchKind, Color, CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
    CompositionNineGridBrush, CompositionScopedBatch, CompositionSpriteShape, ContainerVisual,
    ShapeVisual, SpriteVisual, Vector3KeyFrameAnimation,
};
use windows_core::{Interface, Result};

#[cfg(feature = "system")]
use crate::DesktopWindowTarget;

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
    #[cfg(feature = "system")]
    pub fn new() -> Result<Self> {
        Ok(Self(bindings::Compositor::new()?))
    }

    /// Wraps a lifted compositor obtained from a WinUI host element.
    ///
    /// This is the interop seam used by the reactor bridge: a WinUI element's
    /// `Microsoft.UI.Composition.Compositor` (surfaced as an
    /// [`IInspectable`](windows_core::IInspectable)) is adopted here so its
    /// visual tree can be built with this crate's safe API. Lifted composition
    /// can only be hosted inside a WinUI element, so this has no system-stack
    /// counterpart.
    #[cfg(feature = "reactor")]
    pub fn from_host(compositor: windows_core::IInspectable) -> Result<Self> {
        Ok(Self(compositor.cast()?))
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
    #[cfg(feature = "system")]
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
    #[cfg(feature = "system")]
    pub unsafe fn create_desktop_window_target_for_hwnd(
        &self,
        hwnd: *mut core::ffi::c_void,
        is_topmost: bool,
    ) -> Result<DesktopWindowTarget> {
        let interop: bindings::ICompositorDesktopInterop = self.0.cast()?;
        let target = unsafe { interop.CreateDesktopWindowTarget(hwnd, is_topmost)? };
        Ok(DesktopWindowTarget::new(target))
    }

    /// Creates an empty container visual that hosts a child visual tree.
    pub fn create_container_visual(&self) -> ContainerVisual {
        ContainerVisual::new(self.0.CreateContainerVisual().unwrap())
    }

    /// Creates a sprite visual that paints itself with a brush.
    pub fn create_sprite_visual(&self) -> SpriteVisual {
        SpriteVisual::new(self.0.CreateSpriteVisual().unwrap())
    }

    /// Creates a solid-color brush.
    pub fn create_color_brush(&self, color: Color) -> CompositionColorBrush {
        CompositionColorBrush(self.0.CreateColorBrushWithColor(color.0).unwrap())
    }

    /// Creates a nine-grid brush.
    pub fn create_nine_grid_brush(&self) -> CompositionNineGridBrush {
        let compositor: bindings::ICompositor2 = self.0.cast().unwrap();
        CompositionNineGridBrush(compositor.CreateNineGridBrush().unwrap())
    }

    /// Creates a shape visual that renders composition shapes.
    pub fn create_shape_visual(&self) -> ShapeVisual {
        let compositor: bindings::ICompositor5 = self.0.cast().unwrap();
        ShapeVisual::new(compositor.CreateShapeVisual().unwrap())
    }

    /// Creates an empty container shape that groups child shapes.
    pub fn create_container_shape(&self) -> CompositionContainerShape {
        let compositor: bindings::ICompositor5 = self.0.cast().unwrap();
        CompositionContainerShape(compositor.CreateContainerShape().unwrap())
    }

    /// Creates an ellipse geometry.
    pub fn create_ellipse_geometry(&self) -> CompositionEllipseGeometry {
        let compositor: bindings::ICompositor5 = self.0.cast().unwrap();
        CompositionEllipseGeometry(compositor.CreateEllipseGeometry().unwrap())
    }

    /// Creates a sprite shape that fills the given geometry with a brush.
    pub fn create_sprite_shape(
        &self,
        geometry: &CompositionEllipseGeometry,
    ) -> CompositionSpriteShape {
        let compositor: bindings::ICompositor5 = self.0.cast().unwrap();
        CompositionSpriteShape(
            compositor
                .CreateSpriteShapeWithGeometry(&geometry.as_geometry().0)
                .unwrap(),
        )
    }

    /// Creates a scoped batch that tracks the completion of the given kind of
    /// work.
    pub fn create_scoped_batch(&self, kind: BatchKind) -> CompositionScopedBatch {
        CompositionScopedBatch(self.0.CreateScopedBatch(kind.into()).unwrap())
    }

    /// Creates a `Vector3` key-frame animation.
    pub fn create_vector3_key_frame_animation(&self) -> Vector3KeyFrameAnimation {
        Vector3KeyFrameAnimation(self.0.CreateVector3KeyFrameAnimation().unwrap())
    }
}

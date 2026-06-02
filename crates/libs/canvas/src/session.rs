use std::cell::Cell;

use crate::bindings::*;
use crate::bitmap::Bitmap;
use crate::color::Color;
use crate::device_lost;
use crate::geometry::Path;
use crate::text::TextFormat;
use crate::types::{
    Brush, Ellipse, GradientStop, LinearGradient, Paint, RadialGradient, Rect, RoundedRect,
};
use windows_numerics::Vector2;

/// Safe wrapper over `ID2D1DeviceContext`.
///
/// Calls `BeginDraw` on creation and `EndDraw` on drop.
/// All drawing methods are safe — no UB is possible through this API.
pub struct DrawingSession<'a> {
    context: &'a ID2D1DeviceContext,
    device_lost_flag: &'a Cell<bool>,
}

impl<'a> DrawingSession<'a> {
    pub(crate) fn new(
        context: &'a ID2D1DeviceContext,
        device_lost_flag: &'a Cell<bool>,
    ) -> windows_core::Result<Self> {
        unsafe { context.BeginDraw() };
        Ok(Self {
            context,
            device_lost_flag,
        })
    }

    /// Clear the render target to the specified color.
    pub fn clear(&self, color: Color) {
        let c: D2D1_COLOR_F = color.into();
        unsafe { self.context.Clear(Some(&c)) };
    }

    /// Draw a line between two points.
    pub fn draw_line(&self, p0: Vector2, p1: Vector2, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawLine(p0, p1, brush.as_raw_brush(), width, None);
        }
    }

    /// Draw a rectangle outline.
    pub fn draw_rect(&self, rect: &Rect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Fill a rectangle.
    pub fn fill_rect(&self, rect: &Rect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    /// Draw a rounded rectangle outline.
    pub fn draw_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRoundedRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Fill a rounded rectangle.
    pub fn fill_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRoundedRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    /// Draw an ellipse outline.
    pub fn draw_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawEllipse(&ellipse.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Fill an ellipse.
    pub fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint) {
        unsafe {
            self.context
                .FillEllipse(&ellipse.to_abi(), brush.as_raw_brush());
        }
    }

    /// Create a solid color brush bound to this session's device context.
    pub fn create_solid_brush(&self, color: Color) -> windows_core::Result<Brush> {
        let c: D2D1_COLOR_F = color.into();
        unsafe { self.context.CreateSolidColorBrush(&c, None).map(Brush) }
    }

    /// Create a linear gradient brush.
    ///
    /// `start` and `end` define the gradient axis in local coordinates.
    /// `stops` defines colors at positions 0.0–1.0 along the axis.
    pub fn create_linear_gradient(
        &self,
        start: Vector2,
        end: Vector2,
        stops: &[GradientStop],
    ) -> windows_core::Result<LinearGradient> {
        let abi_stops: Vec<D2D1_GRADIENT_STOP> = stops.iter().map(|s| s.to_abi()).collect();
        unsafe {
            let collection = self.context.CreateGradientStopCollection(
                &abi_stops,
                D2D1_GAMMA_2_2,
                D2D1_EXTEND_MODE_CLAMP,
            )?;
            let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: start,
                endPoint: end,
            };
            self.context
                .CreateLinearGradientBrush(&props, None, &collection)
                .map(LinearGradient)
        }
    }

    /// Create a radial gradient brush.
    ///
    /// `center` is the gradient center. `radius_x`/`radius_y` define the ellipse.
    /// `stops` defines colors at positions 0.0 (center) to 1.0 (edge).
    pub fn create_radial_gradient(
        &self,
        center: Vector2,
        radius_x: f32,
        radius_y: f32,
        stops: &[GradientStop],
    ) -> windows_core::Result<RadialGradient> {
        let abi_stops: Vec<D2D1_GRADIENT_STOP> = stops.iter().map(|s| s.to_abi()).collect();
        unsafe {
            let collection = self.context.CreateGradientStopCollection(
                &abi_stops,
                D2D1_GAMMA_2_2,
                D2D1_EXTEND_MODE_CLAMP,
            )?;
            let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center,
                gradientOriginOffset: Vector2::new(0.0, 0.0),
                radiusX: radius_x,
                radiusY: radius_y,
            };
            self.context
                .CreateRadialGradientBrush(&props, None, &collection)
                .map(RadialGradient)
        }
    }

    /// Draw text within a layout rectangle.
    pub fn draw_text(&self, text: &str, format: &TextFormat, rect: &Rect, brush: &impl Paint) {
        let wide: Vec<u16> = text.encode_utf16().collect();
        unsafe {
            self.context.DrawText(
                &wide,
                format.raw(),
                &rect.to_abi(),
                brush.as_raw_brush(),
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                0,
            );
        }
    }

    /// Draw a path geometry outline.
    pub fn draw_path(&self, path: &Path, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawGeometry(path.raw(), brush.as_raw_brush(), width, None);
        }
    }

    /// Fill a path geometry.
    pub fn fill_path(&self, path: &Path, brush: &impl Paint) {
        unsafe {
            self.context
                .FillGeometry(path.raw(), brush.as_raw_brush(), None);
        }
    }

    /// Draw a bitmap at the specified destination rectangle.
    ///
    /// The bitmap is scaled to fill `dest`. Use `opacity` (0.0–1.0) to blend.
    pub fn draw_bitmap(&self, bitmap: &Bitmap, dest: &Rect, opacity: f32) {
        unsafe {
            self.context.DrawBitmap(
                &bitmap.0,
                Some(&dest.to_abi()),
                opacity,
                D2D1_INTERPOLATION_MODE_LINEAR,
                None,
                None,
            );
        }
    }

    /// Load a bitmap from an image file (PNG, JPEG, BMP, etc.).
    ///
    /// Requires COM to be initialized. For best performance, load bitmaps once
    /// (e.g. via [`SwapChain::load_bitmap`]) and reuse across frames.
    pub fn load_bitmap(&self, path: impl AsRef<std::path::Path>) -> windows_core::Result<Bitmap> {
        Bitmap::load_from_file(self.context, path.as_ref())
    }

    /// Set the current transform matrix.
    pub fn set_transform(&self, transform: &windows_numerics::Matrix3x2) {
        unsafe { self.context.SetTransform(transform) };
    }

    /// Get the current transform matrix.
    pub fn get_transform(&self) -> windows_numerics::Matrix3x2 {
        let mut transform = windows_numerics::Matrix3x2::default();
        unsafe { self.context.GetTransform(&mut transform) };
        transform
    }

    /// Execute a closure with a temporary transform, then restore the previous one.
    pub fn with_transform(&self, transform: &windows_numerics::Matrix3x2, f: impl FnOnce()) {
        let prev = self.get_transform();
        self.set_transform(transform);
        f();
        self.set_transform(&prev);
    }

    /// Access the raw D2D1 device context for advanced usage.
    pub fn raw(&self) -> &ID2D1DeviceContext {
        self.context
    }
}

impl Drop for DrawingSession<'_> {
    fn drop(&mut self) {
        unsafe {
            let result = self.context.EndDraw(None, None);
            if let Err(e) = &result
                && device_lost::is_device_lost(e.code())
            {
                self.device_lost_flag.set(true);
            }
        }
    }
}

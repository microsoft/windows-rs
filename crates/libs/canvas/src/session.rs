use std::cell::Cell;

use crate::bindings::*;
use crate::bitmap::Bitmap;
use crate::color::ColorF;
use crate::device_lost;
use crate::effect::Effect;
use crate::geometry::Path;
use crate::text::TextFormat;
use crate::types::{
    Brush, Ellipse, GradientStop, LinearGradient, Paint, RadialGradient, Rect, RoundedRect,
    StrokeStyle,
};
use windows_numerics::Vector2;

/// Safe wrapper over `ID2D1DeviceContext`.
///
/// Calls `BeginDraw` on creation and `EndDraw` on drop.
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
    pub fn clear(&self, color: ColorF) {
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

    /// Draw a line with a stroke style.
    pub fn draw_line_styled(
        &self,
        p0: Vector2,
        p1: Vector2,
        brush: &impl Paint,
        width: f32,
        style: &StrokeStyle,
    ) {
        unsafe {
            self.context
                .DrawLine(p0, p1, brush.as_raw_brush(), width, &style.0);
        }
    }

    /// Draw a rectangle outline.
    pub fn draw_rect(&self, rect: &Rect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Draw a rectangle outline with a stroke style.
    pub fn draw_rect_styled(
        &self,
        rect: &Rect,
        brush: &impl Paint,
        width: f32,
        style: &StrokeStyle,
    ) {
        unsafe {
            self.context
                .DrawRectangle(&rect.to_abi(), brush.as_raw_brush(), width, &style.0);
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

    /// Draw a rounded rectangle outline with a stroke style.
    pub fn draw_rounded_rect_styled(
        &self,
        rect: &RoundedRect,
        brush: &impl Paint,
        width: f32,
        style: &StrokeStyle,
    ) {
        unsafe {
            self.context.DrawRoundedRectangle(
                &rect.to_abi(),
                brush.as_raw_brush(),
                width,
                &style.0,
            );
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

    /// Draw an ellipse outline with a stroke style.
    pub fn draw_ellipse_styled(
        &self,
        ellipse: &Ellipse,
        brush: &impl Paint,
        width: f32,
        style: &StrokeStyle,
    ) {
        unsafe {
            self.context
                .DrawEllipse(&ellipse.to_abi(), brush.as_raw_brush(), width, &style.0);
        }
    }

    /// Fill an ellipse.
    pub fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint) {
        unsafe {
            self.context
                .FillEllipse(&ellipse.to_abi(), brush.as_raw_brush());
        }
    }

    /// Create a solid color brush.
    pub fn create_solid_brush(&self, color: ColorF) -> windows_core::Result<Brush> {
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

    /// Draw a path geometry outline with a stroke style.
    pub fn draw_path_styled(
        &self,
        path: &Path,
        brush: &impl Paint,
        width: f32,
        style: &StrokeStyle,
    ) {
        unsafe {
            self.context
                .DrawGeometry(path.raw(), brush.as_raw_brush(), width, &style.0);
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
    /// Requires COM to be initialized. For best performance, load once
    /// and reuse across frames.
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

    /// Create a BGRA8 premultiplied bitmap render target matching the current target's size and DPI.
    pub fn create_bitmap_target(&self) -> windows_core::Result<Bitmap> {
        unsafe {
            let mut dpi_x = 0.0f32;
            let mut dpi_y = 0.0f32;
            self.context.GetDpi(&mut dpi_x, &mut dpi_y);
            let pixel_size = self.context.GetPixelSize();

            let properties = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi_x,
                dpiY: dpi_y,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET,
                ..Default::default()
            };

            self.context
                .CreateBitmap(pixel_size, None, 0, &properties)
                .map(Bitmap)
        }
    }

    /// Create a drop shadow effect for a bitmap render target.
    pub fn create_shadow(&self, source: &Bitmap) -> windows_core::Result<Effect> {
        unsafe {
            let effect = self.context.CreateEffect(&CLSID_D2D1Shadow)?;
            effect.SetInput(0, &source.0, true);
            Ok(Effect(effect))
        }
    }

    /// Temporarily redirect drawing to a bitmap render target.
    ///
    /// Restores the previous target when the closure returns.
    pub fn with_target(&self, bitmap: &Bitmap, f: impl FnOnce()) {
        unsafe {
            let previous = self.context.GetTarget();
            self.context.SetTarget(&bitmap.0);
            f();
            match previous {
                Ok(prev) => self.context.SetTarget(&prev),
                Err(_) => self.context.SetTarget(None::<&ID2D1Image>),
            }
        }
    }

    /// Draw a bitmap at its natural size at the current transform.
    ///
    /// Unlike [`draw_bitmap`](Self::draw_bitmap) which scales to a destination rect.
    pub fn draw_image(&self, bitmap: &Bitmap) {
        unsafe {
            self.context.DrawImage(
                &bitmap.0,
                None,
                None,
                D2D1_INTERPOLATION_MODE_LINEAR,
                0, // D2D1_COMPOSITE_MODE_SOURCE_OVER
            );
        }
    }

    /// Draw an effect's output at the current transform.
    pub fn draw_effect(&self, effect: &Effect) {
        if let Ok(output) = unsafe { effect.0.GetOutput() } {
            unsafe {
                self.context.DrawImage(
                    &output,
                    None,
                    None,
                    D2D1_INTERPOLATION_MODE_LINEAR,
                    0, // D2D1_COMPOSITE_MODE_SOURCE_OVER
                );
            }
        }
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

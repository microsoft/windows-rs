use super::*;

/// Safe wrapper over `ID2D1DeviceContext`.
///
/// A session drives either a swap chain (owning the `BeginDraw`/`EndDraw`
/// bracket) or a borrowed context that is already in a drawing state, such as the
/// one handed back by a `SurfaceImageSource`. In the borrowed case an `offset`
/// translation shifts every draw into the shared atlas surface; it composes with
/// caller transforms so `set_transform`/`with_transform` behave as if the surface
/// origin were `(0, 0)`.
pub struct DrawingSession<'a> {
    context: &'a ID2D1DeviceContext,
    mode: Mode<'a>,
}

/// How a [`DrawingSession`] relates to its `ID2D1DeviceContext`.
enum Mode<'a> {
    /// The session owns the `BeginDraw`/`EndDraw` bracket (swap-chain path) and
    /// flags device loss reported by `EndDraw`.
    Owned { device_lost_flag: &'a Cell<bool> },
    /// The context is already in a drawing state and is bracketed by its owner
    /// (e.g. a `SurfaceImageSource`), so the session neither begins nor ends
    /// drawing. `offset` is a pure translation mapping surface-local coordinates
    /// onto the shared-atlas position.
    Borrowed { offset: Matrix3x2 },
}

impl<'a> DrawingSession<'a> {
    pub(crate) fn new(
        context: &'a ID2D1DeviceContext,
        device_lost_flag: &'a Cell<bool>,
    ) -> Result<Self> {
        unsafe { context.BeginDraw() };
        Ok(Self {
            context,
            mode: Mode::Owned { device_lost_flag },
        })
    }

    /// Wrap an `ID2D1DeviceContext` that is *already in a drawing state* — for
    /// example the one returned by `ISurfaceImageSourceNativeWithD2D::BeginDraw`
    /// or a printing/interop context you drive yourself.
    ///
    /// The session neither begins nor ends drawing: the caller owns the
    /// `BeginDraw`/`EndDraw` bracket. `offset` is a pure translation applied
    /// beneath every draw (composing with caller transforms), so you draw from a
    /// `(0, 0)` origin even when the target is a sub-region of a shared surface;
    /// pass `Matrix3x2::translation(0.0, 0.0)` for no offset.
    pub fn from_borrowed_context(context: &'a ID2D1DeviceContext, offset: Matrix3x2) -> Self {
        debug_assert!(
            offset.m11 == 1.0 && offset.m12 == 0.0 && offset.m21 == 0.0 && offset.m22 == 1.0,
            "offset must be a pure translation: get_transform decomposes it by negating m31/m32"
        );
        unsafe { context.SetTransform(&offset) };
        Self {
            context,
            mode: Mode::Borrowed { offset },
        }
    }

    /// Clears the entire session to the given color.
    pub fn clear(&self, color: ColorF) {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.context.Clear(Some(&c)) };
    }

    /// Draws a straight line between two points.
    pub fn draw_line(&self, p0: Vector2, p1: Vector2, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawLine(p0, p1, brush.as_raw_brush(), width, None);
        }
    }

    /// Draws a straight line using the given stroke style.
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

    /// Draws the outline of a rectangle.
    pub fn draw_rect(&self, rect: &Rect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Draws the outline of a rectangle using the given stroke style.
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

    /// Fills a rectangle.
    pub fn fill_rect(&self, rect: &Rect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    /// Draws the outline of a rounded rectangle.
    pub fn draw_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRoundedRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Draws the outline of a rounded rectangle using the given stroke style.
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

    /// Fills a rounded rectangle.
    pub fn fill_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRoundedRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    /// Draws the outline of an ellipse.
    pub fn draw_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawEllipse(&ellipse.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

    /// Draws the outline of an ellipse using the given stroke style.
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

    /// Fills an ellipse.
    pub fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint) {
        unsafe {
            self.context
                .FillEllipse(&ellipse.to_abi(), brush.as_raw_brush());
        }
    }

    /// Creates a solid color brush.
    pub fn create_solid_brush(&self, color: ColorF) -> Result<Brush> {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.context.CreateSolidColorBrush(&c, None).map(Brush) }
    }

    /// Stops define colors at positions 0.0–1.0 along the axis from `start` to `end`.
    pub fn create_linear_gradient(
        &self,
        start: Vector2,
        end: Vector2,
        stops: &[GradientStop],
    ) -> Result<LinearGradient> {
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

    /// Stops define colors at positions 0.0 (center) to 1.0 (edge).
    pub fn create_radial_gradient(
        &self,
        center: Vector2,
        radius_x: f32,
        radius_y: f32,
        stops: &[GradientStop],
    ) -> Result<RadialGradient> {
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

    /// Draws text within a rectangle using the given format and brush.
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

    /// Draws the outline of a path.
    pub fn draw_path(&self, path: &Path, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawGeometry(path.raw(), brush.as_raw_brush(), width, None);
        }
    }

    /// Draws the outline of a path using the given stroke style.
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

    /// Fills a path.
    pub fn fill_path(&self, path: &Path, brush: &impl Paint) {
        unsafe {
            self.context
                .FillGeometry(path.raw(), brush.as_raw_brush(), None);
        }
    }

    /// Draws a bitmap into the destination rectangle with the given opacity.
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

    /// Loads a bitmap from an image file.
    pub fn load_bitmap(&self, path: impl AsRef<std::path::Path>) -> Result<Bitmap> {
        Bitmap::load_from_file(self.context, path.as_ref())
    }

    /// Sets the current transform.
    pub fn set_transform(&self, transform: &Matrix3x2) {
        let m = match &self.mode {
            Mode::Borrowed { offset } => *transform * *offset,
            Mode::Owned { .. } => *transform,
        };
        unsafe { self.context.SetTransform(&m) };
    }

    /// Returns the current transform.
    pub fn get_transform(&self) -> Matrix3x2 {
        let mut transform = Matrix3x2::default();
        unsafe { self.context.GetTransform(&mut transform) };
        match &self.mode {
            // Undo the atlas offset (a pure translation) so callers see the
            // surface origin as `(0, 0)`.
            Mode::Borrowed { offset } => {
                transform * Matrix3x2::translation(-offset.m31, -offset.m32)
            }
            Mode::Owned { .. } => transform,
        }
    }

    /// Apply a transform for the duration of the closure, then restore the previous one.
    pub fn with_transform(&self, transform: &Matrix3x2, f: impl FnOnce()) {
        let prev = self.get_transform();
        self.set_transform(transform);
        f();
        self.set_transform(&prev);
    }

    /// Returns the underlying `ID2D1DeviceContext`.
    pub fn raw(&self) -> &ID2D1DeviceContext {
        self.context
    }

    /// Creates a bitmap suitable for use as a render target.
    pub fn create_bitmap_target(&self) -> Result<Bitmap> {
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

    /// Creates a shadow effect from the given bitmap.
    pub fn create_shadow(&self, source: &Bitmap) -> Result<Effect> {
        unsafe {
            let effect = self.context.CreateEffect(&CLSID_D2D1Shadow)?;
            effect.SetInput(0, &source.0, true);
            Ok(Effect(effect))
        }
    }

    /// Redirect drawing to a bitmap target for the duration of the closure.
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

    /// Draws the output of an effect.
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
        // A borrowed context is ended by its owner, not here.
        let Mode::Owned { device_lost_flag } = self.mode else {
            return;
        };
        unsafe {
            let result = self.context.EndDraw(None, None);
            if is_device_lost(result) {
                device_lost_flag.set(true);
            }
        }
    }
}

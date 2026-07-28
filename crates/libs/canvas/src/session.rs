use super::*;

/// Safe wrapper over `ID2D1DeviceContext`.
pub struct DrawingSession<'a> {
    context: &'a ID2D1DeviceContext,
    mode: Mode<'a>,
}

enum Mode<'a> {
    /// This session owns the `BeginDraw`/`EndDraw` bracket.
    Owned { device_lost_flag: &'a Cell<bool> },
    /// This borrowed context is already bracketed by its owner.
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

    /// Wraps a context that is already inside a caller-owned `BeginDraw`/`EndDraw` bracket.
    ///
    /// `offset` maps surface-local coordinates into a shared atlas while keeping
    /// caller-visible transforms relative to `(0, 0)`.
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

    /// Wraps a borrowed context whose coordinates are device-independent pixels at `dpi`.
    pub fn from_borrowed_context_with_dpi(
        context: &'a ID2D1DeviceContext,
        offset: Matrix3x2,
        dpi: f32,
    ) -> Self {
        unsafe { context.SetDpi(dpi, dpi) };
        Self::from_borrowed_context(context, offset)
    }

    pub fn clear(&self, color: ColorF) {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.context.Clear(Some(&c)) };
    }

    pub fn draw_line(&self, p0: Vector2, p1: Vector2, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawLine(p0, p1, brush.as_raw_brush(), width, None);
        }
    }

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

    pub fn draw_rect(&self, rect: &Rect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

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

    pub fn fill_rect(&self, rect: &Rect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    pub fn draw_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawRoundedRectangle(&rect.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

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

    pub fn fill_rounded_rect(&self, rect: &RoundedRect, brush: &impl Paint) {
        unsafe {
            self.context
                .FillRoundedRectangle(&rect.to_abi(), brush.as_raw_brush());
        }
    }

    pub fn draw_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawEllipse(&ellipse.to_abi(), brush.as_raw_brush(), width, None);
        }
    }

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

    pub fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl Paint) {
        unsafe {
            self.context
                .FillEllipse(&ellipse.to_abi(), brush.as_raw_brush());
        }
    }

    pub fn create_solid_brush(&self, color: ColorF) -> Result<Brush> {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.context.CreateSolidColorBrush(&c, None).map(Brush) }
    }

    /// Stops define colors at positions 0.0-1.0 along the axis from `start` to `end`.
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

    /// Draws a pre-shaped [`TextLayout`] with its top-left at `origin`.
    ///
    /// Prefer this over [`draw_text`](Self::draw_text) when the same text is
    /// drawn across frames: the layout is shaped once and reused.
    pub fn draw_text_layout(&self, origin: Vector2, layout: &TextLayout, brush: &impl Paint) {
        unsafe {
            self.context.DrawTextLayout(
                origin,
                layout.raw(),
                brush.as_raw_brush(),
                D2D1_DRAW_TEXT_OPTIONS_NONE,
            );
        }
    }

    pub fn draw_path(&self, path: &Path, brush: &impl Paint, width: f32) {
        unsafe {
            self.context
                .DrawGeometry(path.raw(), brush.as_raw_brush(), width, None);
        }
    }

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

    pub fn fill_path(&self, path: &Path, brush: &impl Paint) {
        unsafe {
            self.context
                .FillGeometry(path.raw(), brush.as_raw_brush(), None);
        }
    }

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

    pub fn load_bitmap(&self, path: impl AsRef<std::path::Path>) -> Result<Bitmap> {
        Bitmap::load_from_file(self.context, path.as_ref())
    }

    /// Creates a bitmap from tightly packed 32-bit premultiplied BGRA pixels.
    pub fn create_bitmap(&self, pixels: &[u8], width: u32, height: u32) -> Result<Bitmap> {
        Bitmap::from_bytes(
            self.context,
            pixels,
            width,
            height,
            AlphaMode::Premultiplied,
        )
    }

    /// Creates a bitmap from tightly packed 32-bit BGRA pixels with an explicit [`AlphaMode`].
    pub fn create_bitmap_with_alpha(
        &self,
        pixels: &[u8],
        width: u32,
        height: u32,
        alpha: AlphaMode,
    ) -> Result<Bitmap> {
        Bitmap::from_bytes(self.context, pixels, width, height, alpha)
    }

    pub fn set_transform(&self, transform: &Matrix3x2) {
        let m = match &self.mode {
            Mode::Borrowed { offset } => *transform * *offset,
            Mode::Owned { .. } => *transform,
        };
        unsafe { self.context.SetTransform(&m) };
    }

    pub fn transform(&self) -> Matrix3x2 {
        let mut transform = Matrix3x2::default();
        unsafe { self.context.GetTransform(&mut transform) };
        match &self.mode {
            Mode::Borrowed { offset } => {
                transform * Matrix3x2::translation(-offset.m31, -offset.m32)
            }
            Mode::Owned { .. } => transform,
        }
    }

    /// Apply a transform for the duration of the closure, then restore the previous one.
    pub fn with_transform(&self, transform: &Matrix3x2, f: impl FnOnce()) {
        let prev = self.transform();
        self.set_transform(transform);
        f();
        self.set_transform(&prev);
    }

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

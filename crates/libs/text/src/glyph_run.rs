use super::*;
use std::ffi::c_void;
use std::ops::Range;
use std::slice::from_raw_parts;
use windows_core::implement_decl;

/// Clockwise orientation of glyphs in the layout.
///
/// DirectWrite separately describes whether glyphs use sideways origins and vertical metrics.
/// Use [`transform`](Self::transform) with [`GlyphRun::is_sideways`] when drawing a native run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum GlyphOrientation {
    /// Upright.
    #[default]
    Degrees0,
    /// Rotated 90 degrees clockwise.
    Degrees90,
    /// Upside down.
    Degrees180,
    /// Rotated 270 degrees clockwise.
    Degrees270,
}

impl GlyphOrientation {
    /// Rotation from native glyph drawing coordinates to layout coordinates.
    ///
    /// DirectWrite draws sideways glyphs 90 degrees counterclockwise using vertical metrics.
    /// This compensates for that rotation before applying the glyph orientation. To draw one
    /// glyph at its [`Glyph::position`], use a zero-advance native glyph run with the same
    /// `isSideways` flag and set this matrix's translation to the position.
    pub fn transform(self, is_sideways: bool) -> windows_numerics::Matrix3x2 {
        let rotation = self.with_sideways(is_sideways);
        let x = rotation.rotate(Vector2::new(1.0, 0.0));
        let y = rotation.rotate(Vector2::new(0.0, 1.0));
        windows_numerics::Matrix3x2 {
            m11: x.x,
            m12: x.y,
            m21: y.x,
            m22: y.y,
            m31: 0.0,
            m32: 0.0,
        }
    }

    fn from_abi(angle: DWRITE_GLYPH_ORIENTATION_ANGLE) -> Self {
        match angle {
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES => Self::Degrees0,
            DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES => Self::Degrees90,
            DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES => Self::Degrees180,
            DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES => Self::Degrees270,
            _ => panic!("invalid glyph orientation"),
        }
    }

    fn with_sideways(self, sideways: bool) -> Self {
        Self::from_abi((self as i32 + i32::from(sideways)) % 4)
    }

    fn add(self, other: Self) -> Self {
        Self::from_abi((self as i32 + other as i32) % 4)
    }

    fn rotate(self, point: Vector2) -> Vector2 {
        match self {
            Self::Degrees0 => point,
            Self::Degrees90 => Vector2::new(-point.y, point.x),
            Self::Degrees180 => Vector2::new(-point.x, -point.y),
            Self::Degrees270 => Vector2::new(point.y, -point.x),
        }
    }
}

/// Shaped glyphs that share a font face, size, and direction.
///
/// Returned by [`TextLayout::glyph_runs`]. Positions are in device-independent pixels relative
/// to the layout box, with y pointing down. They are not snapped to pixels.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct GlyphRun {
    /// The face the glyph ids belong to.
    pub font_face: FontFace,
    /// The em size of the font, in device-independent pixels.
    pub font_size: f32,
    /// The start of the run on its baseline, in layout coordinates.
    pub baseline_origin: Vector2,
    /// The glyph orientation in the layout.
    pub orientation: GlyphOrientation,
    /// Whether native drawing uses sideways glyph origins and vertical metrics. Pass this
    /// flag to DirectWrite when drawing, together with [`GlyphOrientation::transform`].
    pub is_sideways: bool,
    /// `true` if the run advances backward along its baseline, as in Arabic or Hebrew.
    pub is_right_to_left: bool,
    /// UTF-8 byte range of the text the run displays. The ellipsis of trimmed text has an empty
    /// range where the visible text before it ends.
    pub text_range: Range<usize>,
    /// The glyphs in text order.
    pub glyphs: Vec<Glyph>,
}

/// One positioned glyph in a [`GlyphRun`].
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Glyph {
    /// The glyph id in the run's font face.
    pub id: u16,
    /// The native glyph origin in layout coordinates, with its advance and shaping offset
    /// applied. Draw a zero-advance glyph at this point using the run's orientation and
    /// sideways flag. For horizontal non-sideways text this is the left end of its advance.
    pub position: Vector2,
    /// The advance of the glyph along the run's baseline.
    pub advance: f32,
    /// UTF-8 byte offset of the start of the cluster the glyph belongs to. Glyphs of one
    /// cluster, such as a base letter and its marks, share this offset.
    pub text_position: usize,
}

/// The kind of a [`Decoration`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DecorationKind {
    /// A line below the baseline.
    Underline,
    /// A line through the text.
    Strikethrough,
}

/// An underline or strikethrough to draw as a filled rectangle.
///
/// Returned by [`TextLayout::decorations`].
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Decoration {
    /// Whether this is an underline or a strikethrough.
    pub kind: DecorationKind,
    /// The rectangle to fill, relative to the layout box.
    pub bounds: Rect,
}

// Collects the output of `IDWriteTextLayout::Draw`. DirectWrite passes it back to the renderer as
// the client drawing context.
pub(crate) struct Collector<'a> {
    offsets: &'a Offsets,
    runs: Option<Vec<GlyphRun>>,
    decorations: Option<Vec<Decoration>>,
    // The face of the previous run and the face DirectWrite passed for it. Adjacent runs often
    // share a face, which saves a query per run.
    face: Option<(*mut c_void, FontFace)>,
    // The end of the previous run, and whether an inline object, such as a trimming ellipsis, is
    // drawing runs whose text is not part of the layout's text.
    end: usize,
    inline: Option<(Vector2, GlyphOrientation)>,
}

impl<'a> Collector<'a> {
    pub fn glyph_runs(layout: &IDWriteTextLayout, offsets: &'a Offsets) -> Vec<GlyphRun> {
        let mut collector = Self {
            offsets,
            runs: Some(Vec::new()),
            decorations: None,
            face: None,
            end: 0,
            inline: None,
        };
        collector.draw(layout);
        collector.runs.unwrap()
    }

    pub fn decorations(layout: &IDWriteTextLayout, offsets: &'a Offsets) -> Vec<Decoration> {
        let mut collector = Self {
            offsets,
            runs: None,
            decorations: Some(Vec::new()),
            face: None,
            end: 0,
            inline: None,
        };
        collector.draw(layout);
        collector.decorations.unwrap()
    }

    // The cached face keeps its object alive, so a matching pointer is the same face.
    fn font_face(&mut self, face: &IDWriteFontFace) -> FontFace {
        match &self.face {
            Some((raw, cached)) if *raw == face.as_raw() => cached.clone(),
            _ => {
                let cached = FontFace(face.cast().unwrap());
                self.face = Some((face.as_raw(), cached.clone()));
                cached
            }
        }
    }

    fn draw(&mut self, layout: &IDWriteTextLayout) {
        let renderer: IDWriteTextRenderer1 = Renderer.into();
        let context: *mut Self = self;
        unsafe {
            layout
                .Draw(Some(context as *const c_void), &renderer, 0.0, 0.0)
                .unwrap();
        }
    }
}

fn glyph_run(
    collector: &mut Collector,
    font_face: FontFace,
    origin: Vector2,
    orientation: GlyphOrientation,
    run: &DWRITE_GLYPH_RUN,
    description: &DWRITE_GLYPH_RUN_DESCRIPTION,
) -> GlyphRun {
    let count = run.glyphCount as usize;
    let is_right_to_left = run.bidiLevel & 1 == 1;
    let (ids, advances, offsets) = unsafe {
        (
            slice(run.glyphIndices, count),
            slice(run.glyphAdvances, count),
            slice(run.glyphOffsets, count),
        )
    };

    let is_sideways = run.isSideways.as_bool();
    let (origin, orientation) = match collector.inline {
        Some((offset, rotation)) => (offset + rotation.rotate(origin), rotation.add(orientation)),
        None => (origin, orientation),
    };
    let rotation = orientation.with_sideways(is_sideways);
    let mut pen = 0.0;
    let mut glyphs: Vec<Glyph> = (0..count)
        .map(|index| {
            let advance = advances[index];
            let offset = offsets.get(index).copied().unwrap_or_default();

            // A right-to-left glyph occupies the advance to the left of the pen, and its
            // offset moves it in the reading direction.
            let x = if is_right_to_left {
                pen -= advance;
                pen - offset.advanceOffset
            } else {
                let x = pen + offset.advanceOffset;
                pen += advance;
                x
            };

            Glyph {
                id: ids[index],
                position: origin + rotation.rotate(Vector2::new(x, -offset.ascenderOffset)),
                advance,
                text_position: 0,
            }
        })
        .collect();

    if collector.inline.is_some() {
        let end = collector.end;
        for glyph in &mut glyphs {
            glyph.text_position = end;
        }
        return GlyphRun {
            font_face,
            font_size: run.fontEmSize,
            baseline_origin: origin,
            orientation,
            is_sideways,
            is_right_to_left,
            text_range: end..end,
            glyphs,
        };
    }

    // The cluster map gives the first glyph of the cluster at each UTF-16 position in the
    // run. A cluster owns the glyphs up to the first glyph of the next cluster.
    let start = description.textPosition;
    let length = description.stringLength as usize;
    let map = unsafe { slice(description.clusterMap, length) };
    let mut cursor = collector.offsets.cursor();
    let run_start = cursor.utf8(start);
    let mut position = 0;

    while position < length {
        let first = map[position];
        let next = map[position..]
            .iter()
            .position(|&glyph| glyph != first)
            .map_or(length, |len| position + len);
        // A run without glyphs, such as tabs, may still number its clusters.
        let last = map
            .get(next)
            .map_or(count, |&glyph| glyph as usize)
            .min(count);
        let text_position = cursor.utf8(start + position as u32);

        for glyph in &mut glyphs[(first as usize).min(last)..last] {
            glyph.text_position = text_position;
        }

        position = next;
    }

    let end = cursor.utf8(start + length as u32);
    collector.end = end;

    GlyphRun {
        font_face,
        font_size: run.fontEmSize,
        baseline_origin: origin,
        orientation,
        is_sideways,
        is_right_to_left,
        text_range: run_start..end,
        glyphs,
    }
}

// Reads a DirectWrite array, which may be null when empty or unused.
unsafe fn slice<'a, T>(data: *const T, len: usize) -> &'a [T] {
    if data.is_null() {
        &[]
    } else {
        unsafe { from_raw_parts(data, len) }
    }
}

unsafe fn collector<'a>(context: *const c_void) -> &'a mut Collector<'a> {
    unsafe { &mut *(context as *mut Collector<'a>) }
}

struct Renderer;

implement_decl! {
    impl Renderer as Renderer_Impl: [IDWriteTextRenderer1]
}

impl IDWritePixelSnapping_Impl for Renderer_Impl {
    fn IsPixelSnappingDisabled(&self, _context: *const c_void, disabled: *mut BOOL) -> HRESULT {
        unsafe { *disabled = true.into() };
        HRESULT(0)
    }

    fn GetCurrentTransform(
        &self,
        _context: *const c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> HRESULT {
        unsafe {
            *transform = DWRITE_MATRIX {
                m11: 1.0,
                m22: 1.0,
                ..Default::default()
            };
        }
        HRESULT(0)
    }

    fn GetPixelsPerDip(&self, _context: *const c_void, pixels: *mut f32) -> HRESULT {
        unsafe { *pixels = 1.0 };
        HRESULT(0)
    }
}

impl IDWriteTextRenderer_Impl for Renderer_Impl {
    fn DrawGlyphRun(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        mode: DWRITE_MEASURING_MODE,
        run: *const DWRITE_GLYPH_RUN,
        description: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        IDWriteTextRenderer1_Impl::DrawGlyphRun(
            self,
            context,
            x,
            y,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            mode,
            run,
            description,
            effect,
        )
    }

    fn DrawUnderline(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        underline: *const DWRITE_UNDERLINE,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        IDWriteTextRenderer1_Impl::DrawUnderline(
            self,
            context,
            x,
            y,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            underline,
            effect,
        )
    }

    fn DrawStrikethrough(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        IDWriteTextRenderer1_Impl::DrawStrikethrough(
            self,
            context,
            x,
            y,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            strikethrough,
            effect,
        )
    }

    fn DrawInlineObject(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        object: Ref<IDWriteInlineObject>,
        is_sideways: BOOL,
        is_right_to_left: BOOL,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        IDWriteTextRenderer1_Impl::DrawInlineObject(
            self,
            context,
            x,
            y,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            object,
            is_sideways,
            is_right_to_left,
            effect,
        )
    }
}

impl IDWriteTextRenderer1_Impl for Renderer_Impl {
    fn DrawGlyphRun(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        angle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        _mode: DWRITE_MEASURING_MODE,
        run: *const DWRITE_GLYPH_RUN,
        description: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        _effect: Ref<IUnknown>,
    ) -> HRESULT {
        let collector = unsafe { collector(context) };
        if collector.runs.is_some() {
            let origin = Vector2::new(x, y);
            let (run, description) =
                unsafe { (run.as_ref().unwrap(), description.as_ref().unwrap()) };
            let face = collector.font_face(run.fontFace.as_ref().unwrap());
            let run = glyph_run(
                collector,
                face,
                origin,
                GlyphOrientation::from_abi(angle),
                run,
                description,
            );
            collector.runs.as_mut().unwrap().push(run);
        }
        HRESULT(0)
    }

    fn DrawUnderline(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        angle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        underline: *const DWRITE_UNDERLINE,
        _effect: Ref<IUnknown>,
    ) -> HRESULT {
        let u = unsafe { &*underline };
        let collector = unsafe { collector(context) };
        if let Some(decorations) = &mut collector.decorations {
            decorations.push(Decoration {
                kind: DecorationKind::Underline,
                bounds: decoration_bounds(x, y, angle, u.offset, u.width, u.thickness),
            });
        }
        HRESULT(0)
    }

    fn DrawStrikethrough(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        angle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        _effect: Ref<IUnknown>,
    ) -> HRESULT {
        let s = unsafe { &*strikethrough };
        let collector = unsafe { collector(context) };
        if let Some(decorations) = &mut collector.decorations {
            decorations.push(Decoration {
                kind: DecorationKind::Strikethrough,
                bounds: decoration_bounds(x, y, angle, s.offset, s.width, s.thickness),
            });
        }
        HRESULT(0)
    }

    // The only inline object a layout can hold is the trimming ellipsis, which draws its own
    // glyph runs back through this renderer.
    fn DrawInlineObject(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        angle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        object: Ref<IDWriteInlineObject>,
        is_sideways: BOOL,
        is_right_to_left: BOOL,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        if unsafe { collector(context) }.runs.is_none() {
            return HRESULT(0);
        }

        let previous = unsafe { collector(context) }.inline;
        unsafe { collector(context) }.inline =
            Some((Vector2::new(x, y), GlyphOrientation::from_abi(angle)));
        let renderer: IDWriteTextRenderer1 = self.to_interface();
        let result = unsafe {
            object.ok().unwrap().Draw(
                Some(context),
                &renderer,
                0.0,
                0.0,
                is_sideways.as_bool(),
                is_right_to_left.as_bool(),
                effect.as_ref(),
            )
        };
        unsafe { collector(context) }.inline = previous;
        result
    }
}

fn decoration_bounds(
    x: f32,
    y: f32,
    angle: DWRITE_GLYPH_ORIENTATION_ANGLE,
    offset: f32,
    width: f32,
    thickness: f32,
) -> Rect {
    let rotation = GlyphOrientation::from_abi(angle);
    let a = rotation.rotate(Vector2::new(0.0, offset));
    let b = rotation.rotate(Vector2::new(width, offset + thickness));
    Rect::new(
        x + a.x.min(b.x),
        y + a.y.min(b.y),
        x + a.x.max(b.x),
        y + a.y.max(b.y),
    )
}

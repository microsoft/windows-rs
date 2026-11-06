use super::*;
use std::ffi::c_void;
use std::ops::Range;
use std::slice::from_raw_parts;
use windows_core::implement_decl;

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
    /// The start of the run on its baseline. A right-to-left run extends to the left of this
    /// point.
    pub baseline_origin: Vector2,
    /// `true` if the run is read right to left.
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
    /// Where to draw the glyph: the left end of its advance on the baseline, with any shaping
    /// offset applied.
    pub position: Vector2,
    /// The horizontal advance of the glyph.
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
    inline: bool,
}

impl<'a> Collector<'a> {
    pub fn glyph_runs(layout: &IDWriteTextLayout, offsets: &'a Offsets) -> Vec<GlyphRun> {
        let mut collector = Self {
            offsets,
            runs: Some(Vec::new()),
            decorations: None,
            face: None,
            end: 0,
            inline: false,
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
            inline: false,
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
        let renderer: IDWriteTextRenderer = Renderer.into();
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

    let mut pen = origin.x;
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
                position: Vector2::new(x, origin.y - offset.ascenderOffset),
                advance,
                text_position: 0,
            }
        })
        .collect();

    if collector.inline {
        let end = collector.end;
        for glyph in &mut glyphs {
            glyph.text_position = end;
        }
        return GlyphRun {
            font_face,
            font_size: run.fontEmSize,
            baseline_origin: origin,
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
        let last = map.get(next).map_or(count, |&glyph| glyph as usize);
        let text_position = cursor.utf8(start + position as u32);

        for glyph in &mut glyphs[first as usize..last] {
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
    impl Renderer as Renderer_Impl: [IDWriteTextRenderer]
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
            let run = glyph_run(collector, face, origin, run, description);
            collector.runs.as_mut().unwrap().push(run);
        }
        HRESULT(0)
    }

    fn DrawUnderline(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        underline: *const DWRITE_UNDERLINE,
        _effect: Ref<IUnknown>,
    ) -> HRESULT {
        let u = unsafe { &*underline };
        let collector = unsafe { collector(context) };
        if let Some(decorations) = &mut collector.decorations {
            decorations.push(Decoration {
                kind: DecorationKind::Underline,
                bounds: Rect::from_xywh(x, y + u.offset, u.width, u.thickness),
            });
        }
        HRESULT(0)
    }

    fn DrawStrikethrough(
        &self,
        context: *const c_void,
        x: f32,
        y: f32,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        _effect: Ref<IUnknown>,
    ) -> HRESULT {
        let s = unsafe { &*strikethrough };
        let collector = unsafe { collector(context) };
        if let Some(decorations) = &mut collector.decorations {
            decorations.push(Decoration {
                kind: DecorationKind::Strikethrough,
                bounds: Rect::from_xywh(x, y + s.offset, s.width, s.thickness),
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
        object: Ref<IDWriteInlineObject>,
        is_sideways: BOOL,
        is_right_to_left: BOOL,
        effect: Ref<IUnknown>,
    ) -> HRESULT {
        if unsafe { collector(context) }.runs.is_none() {
            return HRESULT(0);
        }

        unsafe { collector(context) }.inline = true;
        let renderer: IDWriteTextRenderer = self.to_interface();
        let result = unsafe {
            object.ok().unwrap().Draw(
                Some(context),
                &renderer,
                x,
                y,
                is_sideways.as_bool(),
                is_right_to_left.as_bool(),
                effect.as_ref(),
            )
        };
        unsafe { collector(context) }.inline = false;
        result
    }
}

use super::*;
use std::mem::{ManuallyDrop, transmute_copy};
use std::slice::from_raw_parts;

/// A glyph rasterized to grayscale coverage.
///
/// Returned by [`FontFace::rasterize`]. Bounds are in physical pixels relative to the pen
/// position on the baseline, with y pointing down, so `top` is negative for ink above the
/// baseline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct GlyphImage {
    /// Left edge of the image relative to the pen.
    pub left: i32,
    /// Top edge of the image relative to the baseline.
    pub top: i32,
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
    /// Coverage from 0 to 255, one byte per pixel, in rows from the top without padding.
    pub data: Vec<u8>,
}

/// One layer of a color glyph, such as an emoji.
///
/// Returned by [`FontFace::color_layers`]. Draw the layers in order, each as the coverage of its
/// glyph filled with its color.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ColorLayer {
    /// The glyph id of the layer in the same font face.
    pub glyph: u16,
    /// The layer's red, green, blue, and alpha, from 0.0 to 1.0 and not premultiplied, or `None`
    /// to use the text's foreground color.
    pub color: Option<[f32; 4]>,
}

// The palette index DirectWrite reports for a layer drawn in the foreground color.
const FOREGROUND: u16 = 0xFFFF;

/// The text rendering settings Windows applies when blending glyph coverage.
///
/// Returned by [`RenderingParams::system`]. A renderer that blends coverage from
/// [`FontFace::rasterize`] with these values matches the weight and contrast of native Windows
/// text.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RenderingParams {
    /// The gamma used when blending text with the background, such as 1.8.
    pub gamma: f32,
    /// The contrast added to grayscale coverage before blending, where 0.0 adds none.
    pub grayscale_enhanced_contrast: f32,
}

impl RenderingParams {
    /// Returns the user's settings for the primary monitor. The settings are read on each call.
    pub fn system() -> Self {
        let params: IDWriteRenderingParams1 =
            unsafe { factory().CreateRenderingParams().unwrap().cast().unwrap() };

        unsafe {
            Self {
                gamma: params.GetGamma(),
                grayscale_enhanced_contrast: params.GetGrayscaleEnhancedContrast(),
            }
        }
    }
}

impl FontFace {
    /// Rasterizes `glyph` at an em size of `size` physical pixels, such as the run's font size
    /// multiplied by the display scale. `offset` is the pen's subpixel position, usually from 0.0
    /// to 1.0 on each axis, so a cache can keep a few variants per glyph. Returns `None` for a
    /// glyph with no ink, such as a space. Panics if `size` is negative.
    ///
    /// The glyph is antialiased in grayscale, using the rendering mode and hinting DirectWrite
    /// recommends for the font at this size; small glyphs are usually antialiased horizontally
    /// only, keeping horizontal edges sharp, and a vertical offset rounds to whole pixels.
    /// Positions are not snapped, matching
    /// [`TextLayout::glyph_runs`]. A color glyph rasterizes as its outline; use
    /// [`color_layers`](Self::color_layers) to draw it in color.
    pub fn rasterize(&self, glyph: u16, size: f32, offset: Vector2) -> Option<GlyphImage> {
        let run = self.run(&glyph, size);
        let mut bounds = RECT::default();
        let mut mode = DWRITE_RENDERING_MODE1::default();
        let mut grid_fit = DWRITE_GRID_FIT_MODE::default();

        unsafe {
            self.0
                .GetRecommendedRenderingMode(
                    size,
                    96.0,
                    96.0,
                    None,
                    false,
                    DWRITE_OUTLINE_THRESHOLD_ANTIALIASED,
                    DWRITE_MEASURING_MODE_NATURAL,
                    None,
                    &mut mode,
                    &mut grid_fit,
                )
                .unwrap();
        }

        // Outline mode is for large glyphs drawn as geometry; an alpha texture needs a raster mode.
        if mode == DWRITE_RENDERING_MODE1_OUTLINE {
            mode = DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC;
        }

        let analysis = unsafe {
            factory()
                .CreateGlyphRunAnalysis(
                    &run,
                    None,
                    mode,
                    DWRITE_MEASURING_MODE_NATURAL,
                    grid_fit,
                    DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE,
                    offset.x,
                    offset.y,
                )
                .unwrap()
        };

        unsafe {
            analysis
                .GetAlphaTextureBounds(DWRITE_TEXTURE_ALIASED_1x1, &mut bounds)
                .unwrap();
        }

        if bounds.right <= bounds.left || bounds.bottom <= bounds.top {
            return None;
        }

        let width = (bounds.right - bounds.left) as u32;
        let height = (bounds.bottom - bounds.top) as u32;
        let mut data = vec![0; width as usize * height as usize];

        unsafe {
            analysis
                .CreateAlphaTexture(
                    DWRITE_TEXTURE_ALIASED_1x1,
                    &bounds,
                    data.as_mut_ptr(),
                    data.len().try_into().unwrap(),
                )
                .unwrap();
        }

        Some(GlyphImage {
            left: bounds.left,
            top: bounds.top,
            width,
            height,
            data,
        })
    }

    /// Returns the color layers of `glyph` from the font's default palette, or `None` if the
    /// glyph has no color layers. Rasterize each layer's glyph with [`rasterize`](Self::rasterize)
    /// at the same size and offset as the base glyph.
    pub fn color_layers(&self, glyph: u16) -> Option<Vec<ColorLayer>> {
        if !self.is_color_font() {
            return None;
        }

        let run = self.run(&glyph, 1.0);
        let layers = unsafe {
            factory().TranslateColorGlyphRun(
                0.0,
                0.0,
                &run,
                None,
                DWRITE_MEASURING_MODE_NATURAL,
                None,
                0,
            )
        };

        let layers = match layers {
            Err(error) if error.code() == DWRITE_E_NOCOLOR => return None,
            layers => layers.unwrap(),
        };

        let mut result = Vec::new();

        loop {
            let mut more = BOOL::default();
            unsafe { layers.MoveNext(&mut more).unwrap() };

            if !more.as_bool() {
                return Some(result);
            }

            let layer = unsafe {
                let mut layer = std::ptr::null();
                layers.GetCurrentRun(&mut layer).unwrap();
                &*layer
            };

            let color = (layer.paletteIndex != FOREGROUND).then_some([
                layer.runColor.r,
                layer.runColor.g,
                layer.runColor.b,
                layer.runColor.a,
            ]);

            let run = &layer.glyphRun;
            let glyphs = unsafe { from_raw_parts(run.glyphIndices, run.glyphCount as usize) };
            result.extend(glyphs.iter().map(|&glyph| ColorLayer { glyph, color }));
        }
    }

    // Describes `glyph` as a one-glyph run. The run borrows the face without adding a reference.
    fn run(&self, glyph: &u16, size: f32) -> DWRITE_GLYPH_RUN {
        let face: &IDWriteFontFace = &self.0;

        DWRITE_GLYPH_RUN {
            fontFace: ManuallyDrop::new(Some(unsafe { transmute_copy(face) })),
            fontEmSize: size,
            glyphCount: 1,
            glyphIndices: glyph,
            glyphAdvances: &0.0,
            ..Default::default()
        }
    }
}

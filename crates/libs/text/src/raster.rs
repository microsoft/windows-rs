use super::*;
use std::mem::{ManuallyDrop, transmute_copy};
use std::slice::from_raw_parts;

/// A glyph rasterized to coverage.
///
/// Returned by [`FontFace::rasterize`] and [`FontFace::rasterize_cleartype`]. Bounds are in physical pixels relative to the pen
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
    /// Coverage from 0 to 255 in rows from the top without padding. Grayscale images have one
    /// byte per pixel; ClearType images have three, the red, green, and blue coverage.
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
/// [`FontFace::rasterize`] or [`FontFace::rasterize_cleartype`] with these values, as described
/// for [`gamma_ratios`](Self::gamma_ratios), matches the weight and contrast of native Windows
/// text.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RenderingParams {
    /// The gamma used when blending text with the background, such as 1.8. See
    /// [`gamma_ratios`](Self::gamma_ratios).
    pub gamma: f32,
    /// The contrast added to grayscale coverage before blending, where 0.0 adds none.
    pub grayscale_enhanced_contrast: f32,
    /// The contrast added to ClearType coverage before blending, where 0.0 adds none.
    pub enhanced_contrast: f32,
    /// How much color ClearType adds, from 0.0 to 1.0. Below 1.0, mix each channel's coverage
    /// with the average of the three: `level * channel + (1.0 - level) * average`.
    pub cleartype_level: f32,
    /// Whether the user has ClearType font smoothing turned on. Use
    /// [`FontFace::rasterize_cleartype`] when it is on and the monitor has
    /// [`Rgb`](PixelGeometry::Rgb) or [`Bgr`](PixelGeometry::Bgr) stripes, and
    /// [`FontFace::rasterize`] otherwise.
    pub cleartype: bool,
    /// The order of the color stripes within the primary monitor's pixels.
    pub pixel_geometry: PixelGeometry,
}

/// The order of the color stripes within a monitor's pixels.
///
/// ClearType images from [`FontFace::rasterize_cleartype`] are in red, green, blue order; swap
/// the red and blue coverage when drawing to a [`Bgr`](Self::Bgr) monitor.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum PixelGeometry {
    /// The pixels have no stripes in a known order. Use [`FontFace::rasterize`] for such a
    /// monitor even if ClearType is on.
    #[default]
    Flat,
    /// Red, green, then blue from left to right.
    Rgb,
    /// Blue, green, then red from left to right.
    Bgr,
}

impl RenderingParams {
    /// Returns the user's settings for the primary monitor. The settings are read on each call.
    pub fn system() -> Self {
        let params: IDWriteRenderingParams1 =
            unsafe { factory().CreateRenderingParams().unwrap().cast().unwrap() };

        let pixel_geometry = match unsafe { params.GetPixelGeometry() } {
            DWRITE_PIXEL_GEOMETRY_RGB => PixelGeometry::Rgb,
            DWRITE_PIXEL_GEOMETRY_BGR => PixelGeometry::Bgr,
            _ => PixelGeometry::Flat,
        };

        let cleartype = system_parameter(SPI_GETFONTSMOOTHING) != 0
            && system_parameter(SPI_GETFONTSMOOTHINGTYPE) == FE_FONTSMOOTHINGCLEARTYPE as u32;

        unsafe {
            Self {
                gamma: params.GetGamma(),
                grayscale_enhanced_contrast: params.GetGrayscaleEnhancedContrast(),
                enhanced_contrast: params.GetEnhancedContrast(),
                cleartype_level: params.GetClearTypeLevel(),
                cleartype,
                pixel_geometry,
            }
        }
    }

    /// Returns the constants of DirectWrite's alpha correction for [`gamma`](Self::gamma).
    ///
    /// Glyph coverage is linear, but Windows blends text as if it were gamma corrected, which
    /// depends on the text color. To match it, adjust each coverage value `a` from 0.0 to 1.0
    /// before blending the straight text color `color` over the background with alpha `a`:
    ///
    /// | Step | Grayscale | ClearType, for each channel |
    /// | --- | --- | --- |
    /// | Level | - | `a = level * a + (1 - level) * average`, with [`cleartype_level`](Self::cleartype_level) |
    /// | Contrast | `k = grayscale_enhanced_contrast * s` | `k = enhanced_contrast * s` |
    /// | | `a = a * (k + 1) / (a * k + 1)` | the same |
    /// | Gamma | `f = 0.25 * red + 0.5 * green + 0.25 * blue` | `f` is the channel of `color` |
    /// | | `a = a + a * (1 - a) * ((g[0] * f + g[1]) * a + g[2] * f + g[3])` | the same |
    ///
    /// Here `g` is the result of this method, and `s = clamp(4 * (0.75 - lightness), 0, 1)`
    /// with `lightness = 0.30 * red + 0.59 * green + 0.11 * blue` reduces the contrast for
    /// light text. Gamma values from 1.0 to 2.2 are supported, rounded to a tenth; 1.0 returns
    /// zeros, which leave the coverage as is.
    pub fn gamma_ratios(&self) -> [f32; 4] {
        // Ratios for gamma 1.0 to 2.2 in steps of 0.1, before the scaling below.
        const RATIOS: [[f32; 4]; 13] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0166, -0.0807, 0.2227, -0.0751],
            [0.0350, -0.1760, 0.4325, -0.1370],
            [0.0543, -0.2821, 0.6302, -0.1876],
            [0.0739, -0.3963, 0.8167, -0.2287],
            [0.0933, -0.5161, 0.9926, -0.2616],
            [0.1121, -0.6395, 1.1588, -0.2877],
            [0.1300, -0.7649, 1.3159, -0.3080],
            [0.1469, -0.8911, 1.4644, -0.3234],
            [0.1627, -1.0170, 1.6051, -0.3347],
            [0.1773, -1.1420, 1.7385, -0.3426],
            [0.1908, -1.2652, 1.8650, -0.3476],
            [0.2031, -1.3864, 1.9851, -0.3501],
        ];

        let index = ((self.gamma * 10.0).round().clamp(10.0, 22.0) - 10.0) as usize;
        let [a, b, c, d] = RATIOS[index];
        let squared = 65536.0 / (255.0 * 255.0);
        let linear = 256.0 / 255.0;
        [a * squared, b * linear, c * squared, d * linear]
    }
}

fn system_parameter(action: i32) -> u32 {
    let mut value = 0u32;

    unsafe {
        SystemParametersInfoW(action as u32, 0, &mut value as *mut u32 as _, 0)
            .ok()
            .unwrap();
    }

    value
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
    ///
    /// This uses a horizontal glyph origin. It does not apply a [`GlyphRun`]'s orientation or
    /// sideways metrics; draw oriented runs through DirectWrite or Direct2D instead.
    pub fn rasterize(&self, glyph: u16, size: f32, offset: Vector2) -> Option<GlyphImage> {
        self.rasterize_with(glyph, size, offset, false)
    }

    /// Rasterizes `glyph` like [`rasterize`](Self::rasterize) but with ClearType antialiasing,
    /// which uses the color stripes within each pixel to sharpen text. The image has three bytes
    /// per pixel, the coverage of the red, green, and blue stripes. Blend each color channel
    /// with its own coverage, swapping red and blue for a [`PixelGeometry::Bgr`] monitor.
    ///
    /// ClearType coverage can only be blended with an opaque background, so use
    /// [`rasterize`](Self::rasterize) for text over transparent or animated content. Draw the
    /// [`color_layers`](Self::color_layers) of a color glyph with [`rasterize`](Self::rasterize).
    pub fn rasterize_cleartype(
        &self,
        glyph: u16,
        size: f32,
        offset: Vector2,
    ) -> Option<GlyphImage> {
        self.rasterize_with(glyph, size, offset, true)
    }

    fn rasterize_with(
        &self,
        glyph: u16,
        size: f32,
        offset: Vector2,
        cleartype: bool,
    ) -> Option<GlyphImage> {
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

        let (antialias, texture) = if cleartype {
            (
                DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
                DWRITE_TEXTURE_CLEARTYPE_3x1,
            )
        } else {
            (
                DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE,
                DWRITE_TEXTURE_ALIASED_1x1,
            )
        };

        let analysis = unsafe {
            factory()
                .CreateGlyphRunAnalysis(
                    &run,
                    None,
                    mode,
                    DWRITE_MEASURING_MODE_NATURAL,
                    grid_fit,
                    antialias,
                    offset.x,
                    offset.y,
                )
                .unwrap()
        };

        unsafe {
            analysis
                .GetAlphaTextureBounds(texture, &mut bounds)
                .unwrap();
        }

        if bounds.right <= bounds.left || bounds.bottom <= bounds.top {
            return None;
        }

        let width = (bounds.right - bounds.left) as u32;
        let height = (bounds.bottom - bounds.top) as u32;
        let channels = if cleartype { 3 } else { 1 };
        let mut data = vec![0; width as usize * height as usize * channels];

        unsafe {
            analysis
                .CreateAlphaTexture(
                    texture,
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

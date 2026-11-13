use crate::bindings::*;
use windows_core::{Interface, Result, w};

pub const TEXT: &str = include_str!(r"..\fixtures\balloon.txt");
pub const CHOICES: [&str; 2] = ["もう一度", "おしまい"];
pub const FONT_SIZE: f32 = 18.0;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Horizontal,
    HorizontalRightToLeft,
    VerticalRightToLeft,
    VerticalLeftToRight,
    Stacked,
}

impl Mode {
    pub const ALL: [Self; 5] = [
        Self::Horizontal,
        Self::HorizontalRightToLeft,
        Self::VerticalRightToLeft,
        Self::VerticalLeftToRight,
        Self::Stacked,
    ];
}

// The reference sets native format properties before creating a layout.
pub fn layout(text: &str, mode: Mode, width: f32, height: f32) -> Result<IDWriteTextLayout2> {
    layout_with(text, mode, width, height, |_, _| {})
}

pub fn layout_with(
    text: &str,
    mode: Mode,
    width: f32,
    height: f32,
    setup: impl FnOnce(&IDWriteFactory, &IDWriteTextFormat1),
) -> Result<IDWriteTextLayout2> {
    unsafe {
        let mut factory: Option<IDWriteFactory> = None;
        DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &IDWriteFactory::IID,
            &mut factory as *mut _ as _,
        )
        .ok()?;
        let factory = factory.unwrap();
        let format: IDWriteTextFormat1 = factory
            .CreateTextFormat(
                w!("Yu Gothic").as_ptr(),
                None,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                FONT_SIZE,
                w!("ja-jp").as_ptr(),
            )?
            .cast()?;
        configure(&format, mode)?;
        setup(&factory, &format);
        let text: Vec<u16> = text.encode_utf16().collect();
        factory
            .CreateTextLayout(
                text.as_ptr(),
                text.len().try_into().unwrap(),
                &format,
                width,
                height,
            )?
            .cast()
    }
}

pub fn format(mode: Mode) -> windows_text::TextFormat {
    use windows_text::{TextFormat, VerticalGlyphOrientation, WritingMode};
    let writing_mode = match mode {
        Mode::Horizontal => WritingMode::HorizontalLeftToRight,
        Mode::HorizontalRightToLeft => WritingMode::HorizontalRightToLeft,
        Mode::VerticalRightToLeft | Mode::Stacked => WritingMode::VerticalRightToLeft,
        Mode::VerticalLeftToRight => WritingMode::VerticalLeftToRight,
    };
    TextFormat::new("Yu Gothic", FONT_SIZE)
        .with_locale("ja-jp")
        .with_writing_mode(writing_mode)
        .with_vertical_glyph_orientation(if matches!(mode, Mode::Stacked) {
            VerticalGlyphOrientation::Stacked
        } else {
            VerticalGlyphOrientation::Default
        })
}

fn configure(format: &IDWriteTextFormat1, mode: Mode) -> Result<()> {
    unsafe {
        let (reading, flow) = match mode {
            Mode::Horizontal => (
                DWRITE_READING_DIRECTION_LEFT_TO_RIGHT,
                DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM,
            ),
            Mode::HorizontalRightToLeft => (
                DWRITE_READING_DIRECTION_RIGHT_TO_LEFT,
                DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM,
            ),
            Mode::VerticalRightToLeft | Mode::Stacked => (
                DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
                DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
            ),
            Mode::VerticalLeftToRight => (
                DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
                DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
            ),
        };
        format.SetReadingDirection(reading).ok()?;
        format.SetFlowDirection(flow).ok()?;
        format
            .SetVerticalGlyphOrientation(if matches!(mode, Mode::Stacked) {
                DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED
            } else {
                DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT
            })
            .ok()?;
        Ok(())
    }
}

#[cfg(test)]
pub fn metrics(layout: &IDWriteTextLayout2) -> DWRITE_TEXT_METRICS1 {
    let mut metrics = DWRITE_TEXT_METRICS1::default();
    unsafe { layout.GetMetrics(&mut metrics).unwrap() };
    metrics
}

pub struct Hit {
    pub metrics: DWRITE_HIT_TEST_METRICS,
    pub inside: bool,
    pub trailing: bool,
}

pub fn hit(layout: &IDWriteTextLayout2, x: f32, y: f32) -> Hit {
    let mut metrics = DWRITE_HIT_TEST_METRICS::default();
    let mut trailing = windows_core::BOOL::default();
    let mut inside = windows_core::BOOL::default();
    unsafe {
        layout
            .HitTestPoint(x, y, &mut trailing, &mut inside, &mut metrics)
            .unwrap();
    }
    Hit {
        metrics,
        inside: inside.as_bool(),
        trailing: trailing.as_bool(),
    }
}

pub fn caret(layout: &IDWriteTextLayout2, mode: Mode, hit: &Hit) -> windows_text::Rect {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut metrics = DWRITE_HIT_TEST_METRICS::default();
    unsafe {
        layout
            .HitTestTextPosition(
                hit.metrics.textPosition,
                hit.trailing,
                &mut x,
                &mut y,
                &mut metrics,
            )
            .unwrap();
    }
    match mode {
        Mode::Horizontal | Mode::HorizontalRightToLeft => {
            windows_text::Rect::from_xywh(x, metrics.top, 0.0, metrics.height)
        }
        _ => windows_text::Rect::from_xywh(metrics.left, y, metrics.width, 0.0),
    }
}

pub fn range(text: &str, needle: &str) -> DWRITE_TEXT_RANGE {
    let start = text.find(needle).unwrap();
    DWRITE_TEXT_RANGE {
        startPosition: text[..start].encode_utf16().count().try_into().unwrap(),
        length: needle.encode_utf16().count().try_into().unwrap(),
    }
}

#[cfg(test)]
pub fn bounds(
    layout: &IDWriteTextLayout2,
    range: DWRITE_TEXT_RANGE,
) -> Vec<DWRITE_HIT_TEST_METRICS> {
    let mut count = 0;
    unsafe {
        let result = layout.HitTestTextRange(
            range.startPosition,
            range.length,
            0.0,
            0.0,
            None,
            0,
            &mut count,
        );
        if result != E_NOT_SUFFICIENT_BUFFER {
            result.unwrap();
        }
        let mut bounds = vec![DWRITE_HIT_TEST_METRICS::default(); count as usize];
        layout
            .HitTestTextRange(
                range.startPosition,
                range.length,
                0.0,
                0.0,
                Some(bounds.as_mut_ptr()),
                count,
                &mut count,
            )
            .unwrap();
        bounds.truncate(count as usize);
        bounds
    }
}

#[cfg(test)]
pub fn choice(hit: &Hit) -> Option<usize> {
    if !hit.inside {
        return None;
    }
    CHOICES.iter().position(|choice| {
        let range = range(TEXT, choice);
        (range.startPosition..range.startPosition + range.length)
            .contains(&hit.metrics.textPosition)
    })
}

#[cfg(test)]
mod tests;

mod public;

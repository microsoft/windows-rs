use super::*;
use windows_text::{
    ParagraphAlignment, TextAlignment, TextLayout, Trimming, WordWrapping, WritingMode,
};

fn render(reference: &IDWriteTextLayout, actual: Option<&TextLayout>) -> Vec<u8> {
    let device = windows_canvas::GpuDevice::new_warp().unwrap();
    let target = device.create_render_target(280, 280).unwrap();
    target
        .draw(|session| {
            session.clear(windows_canvas::ColorF::BLACK);
            let origin = windows_text::Vector2::new(20.0, 20.0);
            if let Some(actual) = actual {
                let brush = session.create_solid_brush(windows_canvas::ColorF::WHITE)?;
                session.draw_text_layout(origin, actual, &brush);
            } else {
                let context = session.as_interface::<ID2D1DeviceContext>();
                unsafe {
                    let brush = context.CreateSolidColorBrush(
                        &D2D1_COLOR_F {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                        None,
                    )?;
                    context.DrawTextLayout(
                        origin,
                        reference,
                        &brush,
                        D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT,
                    );
                }
            }
            Ok(())
        })
        .unwrap();
    target.read_pixels().unwrap()
}

#[test]
fn writing_modes_and_alignment_match_native_drawing() {
    for mode in Mode::ALL {
        for (alignment, paragraph, native_alignment, native_paragraph) in [
            (
                TextAlignment::Leading,
                ParagraphAlignment::Trailing,
                DWRITE_TEXT_ALIGNMENT_LEADING,
                DWRITE_PARAGRAPH_ALIGNMENT_FAR,
            ),
            (
                TextAlignment::Center,
                ParagraphAlignment::Leading,
                DWRITE_TEXT_ALIGNMENT_CENTER,
                DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            ),
            (
                TextAlignment::Trailing,
                ParagraphAlignment::Center,
                DWRITE_TEXT_ALIGNMENT_TRAILING,
                DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            ),
        ] {
            for trim in [false, true] {
                let mut format = format(mode)
                    .with_alignment(alignment)
                    .with_paragraph_alignment(paragraph);
                if trim {
                    format = format
                        .with_word_wrapping(WordWrapping::NoWrap)
                        .with_trimming(Trimming::Character);
                }
                let mut actual = TextLayout::new(TEXT, &format, 240.0, 200.0);
                for (width, height) in [(240.0, 200.0), (140.0, 160.0)] {
                    let case = format!(
                        "{mode:?}, alignment={alignment:?}, paragraph={paragraph:?}, trim={trim}, {width}x{height}"
                    );
                    actual.set_max_size(width, height);
                    let reference =
                        layout_with(TEXT, mode, width, height, |factory, format| unsafe {
                            format.SetTextAlignment(native_alignment).unwrap();
                            format.SetParagraphAlignment(native_paragraph).unwrap();
                            if trim {
                                format
                                    .SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP)
                                    .unwrap();
                                let sign = factory.CreateEllipsisTrimmingSign(format).unwrap();
                                format
                                    .SetTrimming(
                                        &DWRITE_TRIMMING {
                                            granularity: DWRITE_TRIMMING_GRANULARITY_CHARACTER,
                                            ..Default::default()
                                        },
                                        &sign,
                                    )
                                    .unwrap();
                            }
                        })
                        .unwrap();
                    let expected = metrics(&reference);
                    let measured = actual.metrics();
                    assert_eq!(
                        measured.bounds(),
                        windows_text::Rect::from_xywh(
                            expected.base.left,
                            expected.base.top,
                            expected.base.width,
                            expected.base.height,
                        ),
                        "{case}"
                    );
                    assert_eq!(measured.line_count, expected.base.lineCount, "{case}");
                    assert_eq!(
                        measured.height_including_trailing_whitespace,
                        expected.heightIncludingTrailingWhitespace,
                        "{case}"
                    );
                    super::super::compare(&actual, &case);
                    let expected = render(&reference, None);
                    let drawn = render(&reference, Some(&actual));
                    assert!(
                        drawn.iter().zip(expected).all(|(a, b)| a.abs_diff(b) <= 1),
                        "{case}: Canvas output differs from native layout drawing"
                    );
                }
            }
        }
    }
}

#[test]
fn changing_a_realized_writing_mode_preserves_existing_layouts() {
    let original = format(Mode::Horizontal);
    let horizontal = TextLayout::new("日本語 ABC", &original, 300.0, 200.0);
    let metrics = horizontal.metrics();
    let vertical = original
        .clone()
        .with_writing_mode(WritingMode::VerticalRightToLeft);
    let columns = TextLayout::new("日本語 ABC", &vertical, 300.0, 200.0);
    assert_eq!(columns.caret_bounds(0, false).height(), 0.0);
    assert!(columns.caret_bounds(0, false).width() > 0.0);
    let changed = vertical.with_writing_mode(WritingMode::HorizontalLeftToRight);
    assert_eq!(
        TextLayout::new("日本語 ABC", &changed, 300.0, 200.0).metrics(),
        metrics
    );
    assert_eq!(
        TextLayout::new("日本語 ABC", &original, 300.0, 200.0).metrics(),
        metrics
    );
    assert_eq!(horizontal.metrics(), metrics);
    assert_eq!(columns.caret_bounds(0, false).height(), 0.0);
}

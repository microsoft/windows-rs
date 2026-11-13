#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), r"\bindings.rs"));
}
use bindings::*;
mod reference;
use std::mem::ManuallyDrop;
use windows_core::Interface;
use windows_text::{
    GlyphOrientation, LineSpacing, Rect, TextFormat, TextLayout, Trimming, Vector2, WordWrapping,
};

#[test]
fn orientation_transform_contract() {
    unsafe {
        let mut factory: Option<IDWriteFactory> = None;
        DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &IDWriteFactory::IID,
            &mut factory as *mut _ as _,
        )
        .unwrap();
        let analyzer: IDWriteTextAnalyzer1 = factory
            .unwrap()
            .CreateTextAnalyzer()
            .unwrap()
            .cast()
            .unwrap();
        for angle in 0..4 {
            for sideways in [false, true] {
                let mut matrix = DWRITE_MATRIX::default();
                analyzer
                    .GetGlyphOrientationTransform(angle, sideways, &mut matrix)
                    .unwrap();
                let orientation = [
                    GlyphOrientation::Degrees0,
                    GlyphOrientation::Degrees90,
                    GlyphOrientation::Degrees180,
                    GlyphOrientation::Degrees270,
                ][angle as usize];
                let actual = orientation.transform(sideways);
                assert_eq!(
                    [
                        actual.m11, actual.m12, actual.m21, actual.m22, actual.m31, actual.m32
                    ],
                    [
                        matrix.m11, matrix.m12, matrix.m21, matrix.m22, matrix.dx, matrix.dy
                    ],
                );
            }
        }
    }
}

fn vertical(text: &str, reading: i32, flow: i32, stacked: bool, trim: bool) -> TextLayout {
    let mut format = TextFormat::new("Yu Gothic", 24.0).with_locale("ja-jp");
    if trim {
        format = format
            .with_trimming(Trimming::Character)
            .with_word_wrapping(WordWrapping::NoWrap);
    }
    let layout = TextLayout::new(text, &format, 240.0, if trim { 90.0 } else { 240.0 });
    configure(&layout, reading, flow, stacked);
    layout
}

fn configure(layout: &TextLayout, reading: i32, flow: i32, stacked: bool) {
    let raw = layout.to_interface::<IDWriteTextFormat1>().unwrap();
    unsafe {
        raw.SetReadingDirection(reading).unwrap();
        raw.SetFlowDirection(flow).unwrap();
        raw.SetVerticalGlyphOrientation(if stacked {
            DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED
        } else {
            DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT
        })
        .unwrap();
    }
}

fn render(layout: &TextLayout, extracted: bool) -> Vec<u8> {
    let device = windows_canvas::GpuDevice::new_warp().unwrap();
    let target = device.create_render_target(280, 280).unwrap();
    target
        .draw(|session| {
            session.clear(windows_canvas::ColorF::BLACK);
            let context = session.as_interface::<ID2D1DeviceContext>();
            unsafe {
                context.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
                let brush = context.CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    None,
                )?;
                if !extracted {
                    context.DrawTextLayout(
                        Vector2::new(20.0, 20.0),
                        layout.as_interface::<IDWriteTextLayout>(),
                        &brush,
                        D2D1_DRAW_TEXT_OPTIONS_NO_SNAP,
                    );
                } else {
                    for run in layout.glyph_runs() {
                        let mut raw = DWRITE_GLYPH_RUN {
                            fontFace: ManuallyDrop::new(Some(
                                run.font_face.to_interface().unwrap(),
                            )),
                            fontEmSize: run.font_size,
                            glyphCount: 1,
                            glyphAdvances: &0.0,
                            isSideways: run.is_sideways.into(),
                            ..Default::default()
                        };
                        for glyph in &run.glyphs {
                            raw.glyphIndices = &glyph.id;
                            let mut transform = run.orientation.transform(run.is_sideways);
                            transform.m31 = glyph.position.x + 20.0;
                            transform.m32 = glyph.position.y + 20.0;
                            context.SetTransform(&transform);
                            context.DrawGlyphRun(
                                Vector2::zero(),
                                &raw,
                                &brush,
                                DWRITE_MEASURING_MODE_NATURAL,
                            );
                        }
                        ManuallyDrop::drop(&mut raw.fontFace);
                    }
                    context.SetTransform(&windows_numerics::Matrix3x2::identity());
                    let brush = session.create_solid_brush(windows_canvas::ColorF::WHITE)?;
                    for decoration in layout.decorations() {
                        session.fill_rect(&decoration.bounds.offset(20.0, 20.0), &brush);
                    }
                }
            }
            Ok(())
        })
        .unwrap();
    target.read_pixels().unwrap()
}

fn compare(layout: &TextLayout, case: &str) {
    let expected = render(layout, false);
    let actual = render(layout, true);
    let different = actual
        .iter()
        .zip(&expected)
        .filter(|(a, b)| a.abs_diff(**b) > 1)
        .count();
    assert_eq!(
        different, 0,
        "{case}: channels differ from native layout drawing"
    );
}

#[test]
fn vertical_extraction_matches_native_drawing() {
    let text = include_str!(r"fixtures\vertical.txt");
    let format = TextFormat::new("Yu Gothic", 24.0).with_locale("ja-jp");
    compare(
        &TextLayout::new(text, &format, 240.0, 240.0),
        "horizontal mixed script",
    );
    for reading in [
        DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
        DWRITE_READING_DIRECTION_BOTTOM_TO_TOP,
    ] {
        for flow in [
            DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
            DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
        ] {
            for stacked in [false, true] {
                let layout = vertical(text, reading, flow, stacked, false);
                let runs = layout.glyph_runs();
                assert!(runs.iter().any(|run| run.is_sideways));
                assert!(runs.iter().any(|run| !run.is_sideways));
                let opening = runs.iter().find(|run| run.text_range.contains(&0)).unwrap();
                let nominal = opening.font_face.glyph_index('「').unwrap();
                let mut expected = 0;
                unsafe {
                    opening
                        .font_face
                        .to_interface::<IDWriteFontFace1>()
                        .unwrap()
                        .GetVerticalGlyphVariants(1, &nominal, &mut expected)
                        .unwrap();
                }
                assert_eq!(opening.glyphs[0].id, expected);
                for run in &runs {
                    assert!(layout.text().is_char_boundary(run.text_range.start));
                    assert!(layout.text().is_char_boundary(run.text_range.end));
                    for glyph in &run.glyphs {
                        assert!(run.text_range.contains(&glyph.text_position));
                        assert!(layout.text().is_char_boundary(glyph.text_position));
                    }
                }
                compare(
                    &layout,
                    &format!("mixed script: reading={reading}, flow={flow}, stacked={stacked}"),
                );
            }
        }
    }
}

#[test]
fn vertical_decorations_and_ellipsis_match_native_drawing() {
    for reading in [
        DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
        DWRITE_READING_DIRECTION_BOTTOM_TO_TOP,
    ] {
        for flow in [
            DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
            DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
        ] {
            for trim in [false, true] {
                let mut layout = vertical("日本語 ABC 日本語 ABC", reading, flow, false, trim);
                layout.set_underline(.., true);
                layout.set_strikethrough(.., true);
                let decorations = layout.decorations();
                assert!(!decorations.is_empty());
                assert!(
                    decorations
                        .iter()
                        .all(|d| d.bounds.height() > d.bounds.width())
                );
                if trim {
                    assert!(
                        layout
                            .glyph_runs()
                            .iter()
                            .any(|run| run.text_range.is_empty())
                    );
                }

                compare(
                    &layout,
                    &format!("decorations: reading={reading}, flow={flow}, trim={trim}"),
                );
            }
        }
    }
}

const GEOMETRY_MODES: [(i32, i32); 6] = [
    (
        DWRITE_READING_DIRECTION_LEFT_TO_RIGHT,
        DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM,
    ),
    (
        DWRITE_READING_DIRECTION_RIGHT_TO_LEFT,
        DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP,
    ),
    (
        DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
        DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
    ),
    (
        DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
        DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
    ),
    (
        DWRITE_READING_DIRECTION_BOTTOM_TO_TOP,
        DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
    ),
    (
        DWRITE_READING_DIRECTION_BOTTOM_TO_TOP,
        DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
    ),
];

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

fn rectangle(hit: &DWRITE_HIT_TEST_METRICS) -> Rect {
    Rect::from_xywh(hit.left, hit.top, hit.width, hit.height)
}

fn position(
    raw: &IDWriteTextLayout,
    offset: u32,
    trailing: bool,
) -> (Vector2, DWRITE_HIT_TEST_METRICS) {
    let mut point = Vector2::zero();
    let mut hit = DWRITE_HIT_TEST_METRICS::default();
    unsafe {
        raw.HitTestTextPosition(offset, trailing, &mut point.x, &mut point.y, &mut hit)
            .unwrap();
    }
    (point, hit)
}

fn utf8(text: &str, offset: u32) -> usize {
    text.char_indices()
        .map(|(index, _)| index)
        .chain([text.len()])
        .find(|&index| text[..index].encode_utf16().count() == offset as usize)
        .unwrap()
}

#[test]
fn line_bands_follow_native_flow_and_alignment() {
    let text = "\r\n日本  \r\n\r\nAB\r\n";
    for (reading, flow) in GEOMETRY_MODES {
        let is_vertical = matches!(
            reading,
            DWRITE_READING_DIRECTION_TOP_TO_BOTTOM | DWRITE_READING_DIRECTION_BOTTOM_TO_TOP
        );
        for alignment in [
            DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            DWRITE_PARAGRAPH_ALIGNMENT_FAR,
        ] {
            for spacing in [
                LineSpacing::Default,
                LineSpacing::Uniform {
                    height: 55.0,
                    baseline: 35.0,
                },
            ] {
                let format = TextFormat::new("Yu Gothic", 24.0)
                    .with_locale("ja-jp")
                    .with_line_spacing(spacing);
                let mut layout = TextLayout::new(text, &format, 400.0, 400.0);
                configure(&layout, reading, flow, false);
                layout.set_font_size(2..5, 36.0);
                let raw = layout.to_interface::<IDWriteTextLayout2>().unwrap();
                unsafe { raw.SetParagraphAlignment(alignment).unwrap() };
                let mut native = DWRITE_TEXT_METRICS1::default();
                unsafe { raw.GetMetrics(&mut native).unwrap() };
                let mut expected =
                    vec![DWRITE_LINE_METRICS::default(); native.base.lineCount as usize];
                let mut count = 0;
                unsafe {
                    raw.GetLineMetrics(
                        Some(expected.as_mut_ptr()),
                        expected.len() as u32,
                        &mut count,
                    )
                    .unwrap();
                };
                let lines = layout.line_metrics();
                assert_eq!(lines.len(), count as usize);
                let mut start = 0;
                for (line, native) in lines.iter().zip(expected) {
                    let end = start + native.length;
                    assert_eq!(line.text_range, utf8(text, start)..utf8(text, end));
                    assert_eq!(
                        line.newline_length,
                        utf8(text, end) - utf8(text, end - native.newlineLength)
                    );
                    assert_eq!(
                        line.trailing_whitespace_length,
                        utf8(text, end) - utf8(text, end - native.trailingWhitespaceLength)
                    );
                    let baseline = if matches!(
                        flow,
                        DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP | DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT
                    ) {
                        native.height - native.baseline
                    } else {
                        native.baseline
                    };
                    close(line.baseline, baseline);
                    if is_vertical {
                        close(line.bounds.width(), native.height);
                        assert_eq!((line.bounds.top, line.bounds.bottom), (0.0, 400.0));
                    } else {
                        close(line.bounds.height(), native.height);
                        assert_eq!((line.bounds.left, line.bounds.right), (0.0, 400.0));
                    }
                    start = end;
                }
                assert_eq!(lines.last().unwrap().text_range, text.len()..text.len());
                let min = lines
                    .iter()
                    .map(|line| {
                        if is_vertical {
                            line.bounds.left
                        } else {
                            line.bounds.top
                        }
                    })
                    .reduce(f32::min)
                    .unwrap();
                let max = lines
                    .iter()
                    .map(|line| {
                        if is_vertical {
                            line.bounds.right
                        } else {
                            line.bounds.bottom
                        }
                    })
                    .reduce(f32::max)
                    .unwrap();
                close(
                    min,
                    if is_vertical {
                        native.base.left
                    } else {
                        native.base.top
                    },
                );
                close(
                    max - min,
                    if is_vertical {
                        native.base.width
                    } else {
                        native.base.height
                    },
                );
                for pair in lines.windows(2) {
                    let (first, next) = (pair[0].bounds, pair[1].bounds);
                    match flow {
                        DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM => close(first.bottom, next.top),
                        DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP => close(first.top, next.bottom),
                        DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT => close(first.right, next.left),
                        DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT => close(first.left, next.right),
                        _ => unreachable!(),
                    }
                }
                let runs = layout.glyph_runs();
                let run = runs.iter().find(|run| run.text_range.contains(&2)).unwrap();
                let line = &lines[1];
                let baseline = match flow {
                    DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM => line.bounds.top + line.baseline,
                    DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP => line.bounds.bottom - line.baseline,
                    DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT => line.bounds.left + line.baseline,
                    DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT => line.bounds.right - line.baseline,
                    _ => unreachable!(),
                };
                close(
                    baseline,
                    if is_vertical {
                        run.baseline_origin.x
                    } else {
                        run.baseline_origin.y
                    },
                );
            }
        }
    }
}

#[test]
fn vertical_metrics_carets_and_utf8_hits_match_native() {
    let fixture = include_str!(r"fixtures\vertical.txt");
    for (reading, flow) in GEOMETRY_MODES {
        let is_vertical = matches!(
            reading,
            DWRITE_READING_DIRECTION_TOP_TO_BOTTOM | DWRITE_READING_DIRECTION_BOTTOM_TO_TOP
        );
        for text in [fixture, "", "日本  "] {
            let mut layout = vertical(text, reading, flow, false, false);
            let raw = layout.to_interface::<IDWriteTextLayout2>().unwrap();
            unsafe {
                raw.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)
                    .unwrap();
                raw.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER).unwrap();
            }
            for size in [240.0, 85.0] {
                layout.set_max_size(size, size);
                let actual = layout.metrics();
                let mut expected = DWRITE_TEXT_METRICS1::default();
                unsafe { raw.GetMetrics(&mut expected).unwrap() };
                assert_eq!(
                    actual.bounds(),
                    Rect::from_xywh(
                        expected.base.left,
                        expected.base.top,
                        expected.base.width,
                        expected.base.height
                    )
                );
                assert_eq!(
                    actual.width_including_trailing_whitespace,
                    expected.base.widthIncludingTrailingWhitespace
                );
                assert_eq!(
                    actual.height_including_trailing_whitespace,
                    expected.heightIncludingTrailingWhitespace
                );
                if text == "日本  " {
                    if is_vertical {
                        assert!(actual.height_including_trailing_whitespace > actual.height);
                    } else {
                        assert!(actual.width_including_trailing_whitespace > actual.width);
                    }
                }
                for index in text
                    .char_indices()
                    .map(|(index, _)| index)
                    .chain([text.len()])
                {
                    let offset = text[..index].encode_utf16().count() as u32;
                    for trailing in [false, true] {
                        let (point, native) = position(&raw, offset, trailing);
                        let expected = if is_vertical {
                            Rect::from_xywh(native.left, point.y, native.width, 0.0)
                        } else {
                            Rect::from_xywh(point.x, native.top, 0.0, native.height)
                        };
                        assert_eq!(layout.caret_bounds(index, trailing), expected);
                        let point = Vector2::new(
                            native.left + native.width * 0.25,
                            native.top + native.height * 0.75,
                        );
                        let mut native = DWRITE_HIT_TEST_METRICS::default();
                        let mut trailing = windows_core::BOOL::default();
                        let mut inside = windows_core::BOOL::default();
                        unsafe {
                            raw.HitTestPoint(
                                point.x,
                                point.y,
                                &mut trailing,
                                &mut inside,
                                &mut native,
                            )
                            .unwrap();
                        };
                        let hit = layout.hit_test_point(point);
                        assert_eq!(
                            hit.text_range,
                            utf8(text, native.textPosition)
                                ..utf8(text, native.textPosition + native.length)
                        );
                        assert_eq!(hit.bounds, rectangle(&native));
                        assert_eq!(hit.is_trailing_hit, trailing.as_bool());
                        assert_eq!(hit.is_inside, inside.as_bool());
                    }
                }
                for range in [0..text.len(), 0..0, text.len()..text.len()] {
                    let offset = text[..range.start].encode_utf16().count() as u32;
                    let length = text[range.clone()].encode_utf16().count() as u32;
                    let mut count = 0;
                    let result = unsafe {
                        raw.HitTestTextRange(offset, length, 0.0, 0.0, None, 0, &mut count)
                    };
                    assert_eq!(result, E_NOT_SUFFICIENT_BUFFER);
                    let mut hits = vec![DWRITE_HIT_TEST_METRICS::default(); count as usize];
                    unsafe {
                        raw.HitTestTextRange(
                            offset,
                            length,
                            0.0,
                            0.0,
                            Some(hits.as_mut_ptr()),
                            count,
                            &mut count,
                        )
                        .unwrap();
                    };
                    assert_eq!(
                        layout.range_bounds(range),
                        hits.iter().map(rectangle).collect::<Vec<_>>()
                    );
                }
            }
        }
    }
}
